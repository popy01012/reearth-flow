use std::str::FromStr;
use std::sync::Arc;

use async_zip::base::read::mem::ZipFileReader;
use directories::ProjectDirs;
use futures::AsyncReadExt;
use serde::{Deserialize, Serialize};

use reearth_flow_common::uri::Uri;

use reearth_flow_action::utils::inject_variables_to_scope;
use reearth_flow_action::{
    error::Error, Action, ActionContext, ActionDataframe, ActionResult, ActionValue, DEFAULT_PORT,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZipExtractor {
    path: String,
    output_path: Option<String>,
}

#[async_trait::async_trait]
#[typetag::serde(name = "ZipExtractor")]
impl Action for ZipExtractor {
    async fn run(&self, ctx: ActionContext, inputs: Option<ActionDataframe>) -> ActionResult {
        let inputs = inputs.unwrap_or_default();

        let expr_engine = Arc::clone(&ctx.expr_engine);
        let scope = expr_engine.new_scope();
        inject_variables_to_scope(&inputs, &scope)?;
        let path = expr_engine
            .eval_scope::<String>(&self.path, &scope)
            .map_err(Error::input)?;
        let path = Uri::from_str(path.as_str()).map_err(Error::input)?;

        let storage = ctx.storage_resolver.resolve(&path).map_err(Error::input)?;
        let file_result = storage
            .get(path.path().as_path())
            .await
            .map_err(Error::internal_runtime)?;
        let bytes = file_result.bytes().await.map_err(Error::internal_runtime)?;
        let reader = ZipFileReader::new(bytes.to_vec())
            .await
            .map_err(Error::internal_runtime)?;

        let root_output_path = match &self.output_path {
            Some(output_path) => {
                let path = expr_engine
                    .eval_scope::<String>(output_path, &scope)
                    .map_err(Error::input)?;
                Uri::from_str(path.as_str()).map_err(Error::input)?
            }
            None => {
                let p = ProjectDirs::from("reearth", "flow", "worker")
                    .ok_or(Error::input("No output path uri provided"))?;
                let p = p
                    .data_dir()
                    .to_str()
                    .ok_or(Error::input("Invalid output path uri"))?;
                let p = format!("{}/output/zip-extractor/{}", p, ctx.node_id);
                tokio::fs::create_dir_all(std::path::Path::new(p.as_str())).await?;
                Uri::for_test(format!("file://{}", p).as_str())
            }
        };
        let storage = ctx
            .storage_resolver
            .resolve(&root_output_path)
            .map_err(Error::input)?;
        let mut output = ActionDataframe::new();

        for i in 0..reader.file().entries().len() {
            let entry = reader
                .file()
                .entries()
                .get(i)
                .ok_or(Error::validate("No entry"))?;
            let filename = entry.filename().as_str().map_err(Error::internal_runtime)?;
            if i == 0 {
                let file_uri = filename
                    .split('/')
                    .next()
                    .ok_or(Error::validate("No file name"))?;
                let file_uri = root_output_path
                    .join(file_uri)
                    .map_err(Error::internal_runtime)?;
                output.insert(
                    DEFAULT_PORT.to_string(),
                    Some(ActionValue::String(file_uri.to_string())),
                );
            }
            let outpath = root_output_path
                .join(filename)
                .map_err(Error::internal_runtime)?;
            let entry_is_dir = filename.ends_with('/');
            if entry_is_dir {
                if storage
                    .exists(outpath.path().as_path())
                    .await
                    .map_err(Error::internal_runtime)?
                {
                    continue;
                }
                storage
                    .create_dir(outpath.path().as_path())
                    .await
                    .map_err(Error::internal_runtime)?;
                continue;
            }
            if let Some(p) = outpath.parent() {
                if !storage
                    .exists(p.path().as_path())
                    .await
                    .map_err(Error::internal_runtime)?
                {
                    storage
                        .create_dir(p.path().as_path())
                        .await
                        .map_err(Error::internal_runtime)?;
                }
            }
            let mut entry_reader = reader
                .reader_without_entry(i)
                .await
                .map_err(Error::internal_runtime)?;
            let mut buf = Vec::<u8>::new();
            entry_reader.read_to_end(&mut buf).await?;
            storage
                .put(outpath.path().as_path(), bytes::Bytes::from(buf))
                .await
                .map_err(Error::internal_runtime)?;
        }
        Ok(output)
    }
}
