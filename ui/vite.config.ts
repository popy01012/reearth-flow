import { readFileSync } from "fs";
import { resolve } from "path";

import { TanStackRouterVite } from "@tanstack/router-vite-plugin";
import react from "@vitejs/plugin-react";
import { readEnv } from "read-env";
import { Plugin, UserConfig, defineConfig, loadEnv } from "vite";

import pkg from "./package.json";

export default defineConfig({
  envPrefix: "FLOW_",
  plugins: [TanStackRouterVite(), react(), config()],
  resolve: {
    alias: [{ find: "@flow", replacement: resolve(__dirname, "./src") }],
  },
  test: {
    environment: "jsdom",
  },
} as UserConfig);

function config(): Plugin {
  return {
    name: "flow-config",
    async configureServer(server) {
      const envs = loadEnv(
        server.config.mode,
        server.config.envDir ?? process.cwd(),
        server.config.envPrefix,
      );
      const remoteConfig = envs.FLOW_CONFIG ? await (await fetch(envs.FLOW_CONFIG)).json() : {};

      const configRes = JSON.stringify(
        {
          version: pkg.version,
          ...remoteConfig,
          ...readEnv("FLOW", {
            source: envs,
          }),
          ...loadJSON("./flow-config.json"),
        },
        null,
        2,
      );

      server.middlewares.use((req, res, next) => {
        if (req.method === "GET" && req.url === "/flow_config.json") {
          res.statusCode = 200;
          res.setHeader("Content-Type", "application/json");
          res.write(configRes);
          res.end();
        } else {
          next();
        }
      });
    },
  };
}

function loadJSON(path: string): any {
  try {
    return JSON.parse(readFileSync(path, "utf8")) || {};
  } catch (err) {
    return {};
  }
}
