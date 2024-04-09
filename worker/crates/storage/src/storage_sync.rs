use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bytes::Bytes;
use futures::Stream;
use object_store::ObjectMeta;
use object_store::Result;
use opendal::BlockingReader;
use opendal::Metakey;
use reearth_flow_common::uri::Uri;

use crate::storage::format_object_store_error;
use crate::storage::Storage;

impl Storage {
    pub fn put_sync(&self, location: &Path, bytes: Bytes) -> Result<()> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        self.inner
            .blocking()
            .write(p, bytes)
            .map_err(|err| format_object_store_error(err, p))
    }

    pub fn create_dir_sync(&self, location: &Path) -> Result<()> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        let p = if !p.ends_with('/') {
            format!("{}/", p)
        } else {
            p.to_string()
        };
        self.inner
            .blocking()
            .create_dir(p.as_str())
            .map_err(|err| format_object_store_error(err, p.as_str()))
    }

    pub fn append_sync(&self, location: &Path, bytes: Bytes) -> Result<()> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        let mut w = self
            .inner
            .blocking()
            .writer_with(p)
            .append(true)
            .call()
            .map_err(|err| format_object_store_error(err, p))?;
        w.write(bytes)
            .map_err(|err| format_object_store_error(err, p))
    }

    pub fn get_sync(&self, location: &Path) -> Result<Bytes> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        let mut r = self
            .inner
            .blocking()
            .reader(p)
            .map_err(|err| format_object_store_error(err, p))?;

        let mut buf = Vec::new();
        r.read_to_end(&mut buf)
            .map_err(|err| object_store::Error::Generic {
                store: "IoError",
                source: Box::new(err),
            })?;
        Ok(Bytes::from(buf))
    }

    pub fn exists_sync(&self, location: &Path) -> Result<bool> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        self.inner
            .blocking()
            .is_exist(p)
            .map_err(|err| format_object_store_error(err, p))
    }

    pub fn get_range_sync(&self, location: &Path, range: Range<usize>) -> Result<Bytes> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        let bs = self
            .inner
            .blocking()
            .read_with(p)
            .range(range.start as u64..range.end as u64)
            .call()
            .map_err(|err| format_object_store_error(err, p))?;

        Ok(Bytes::from(bs))
    }

    pub fn head_sync(&self, location: &Path) -> Result<ObjectMeta> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        let meta = self
            .inner
            .blocking()
            .stat(p)
            .map_err(|err| format_object_store_error(err, p))?;

        Ok(ObjectMeta {
            location: object_store::path::Path::parse(p)?,
            last_modified: meta.last_modified().unwrap_or_default(),
            size: meta.content_length() as usize,
            e_tag: None,
            version: None,
        })
    }

    pub fn delete_sync(&self, location: &Path) -> Result<()> {
        let p = location.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", location).into(),
            },
        })?;
        self.inner
            .blocking()
            .delete(p)
            .map_err(|err| format_object_store_error(err, p))?;
        Ok(())
    }

    pub fn list_sync(&self, prefix: Option<&Path>, recursive: bool) -> Result<Vec<Uri>> {
        let p = prefix.ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", prefix).into(),
            },
        })?;
        let path =
            p.to_str()
                .map(|v| format!("{}/", v))
                .ok_or(object_store::Error::InvalidPath {
                    source: object_store::path::Error::InvalidPath {
                        path: format!("{:?}", prefix).into(),
                    },
                })?;
        let ds = self
            .inner
            .blocking()
            .lister_with(&path)
            .recursive(recursive)
            .metakey(Metakey::ContentLength | Metakey::LastModified)
            .call()
            .map_err(|err| format_object_store_error(err, ""))?;
        let result = ds
            .filter_map(|entry| match entry {
                Ok(v) => Some(Uri::for_test(&format!(
                    "{}/{}",
                    self.base_uri.protocol().as_str_with_separator(),
                    v.path()
                ))),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        Ok(result)
    }

    pub fn copy_sync(&self, from: &Path, to: &Path) -> Result<()> {
        let from = from.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", from).into(),
            },
        })?;
        let to = to.to_str().ok_or(object_store::Error::InvalidPath {
            source: object_store::path::Error::InvalidPath {
                path: format!("{:?}", to).into(),
            },
        })?;
        self.inner
            .blocking()
            .copy(from.as_ref(), to.as_ref())
            .map_err(|err| format_object_store_error(err, from))?;
        Ok(())
    }
}

struct OpendalBlockingReader {
    inner: BlockingReader,
}

impl Stream for OpendalBlockingReader {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = match self.inner.next() {
            Some(Ok(v)) => Some(Ok(v)),
            Some(Err(e)) => Some(Err(object_store::Error::Generic {
                store: "IoError",
                source: Box::new(e),
            })),
            None => None,
        };
        Poll::Ready(result)
    }
}