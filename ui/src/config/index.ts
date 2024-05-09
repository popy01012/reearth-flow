export type Config = {
  version?: string;
  brandName?: string;
  devMode?: boolean;
  githubRepoUrl?: string;
  tosUrl?: string;
  documentationUrl?: string;
};

declare global {
  let __APP_VERSION__: string;
  interface Window {
    FLOW_CONFIG?: Config;
  }
}

const defaultConfig: Config = {
  version: "X.X.X",
  brandName: "Re:Earth Flow",
};

export default async function loadConfig() {
  if (window.FLOW_CONFIG) return;

  window.FLOW_CONFIG = defaultConfig;

  const config: Config = {
    ...defaultConfig,
    ...(await (await fetch("/flow_config.json")).json()),
  };

  if (window.FLOW_CONFIG.brandName) {
    document.title = window.FLOW_CONFIG.brandName + " v" + config.version;
  }

  window.FLOW_CONFIG = config;
}

export function config(): Config {
  return window.FLOW_CONFIG ?? {};
}
