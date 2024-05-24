import { config } from ".";

const tenantKey = "reearth_tennant";

export type AuthInfo = {
  auth0ClientId?: string;
  auth0Domain?: string;
  auth0Audience?: string;
  authProvider?: string;
};

function getLogginInTenantName(): string | null {
  const path = window.location.pathname;
  // /auth/<tennant-name>
  if (path.startsWith("/auth/")) {
    const name = path.split("/")[2];
    return name || null;
  }
  return null;
}

function getMultitenantAuthInfo(conf = config()): AuthInfo | undefined {
  if (!conf?.multiTenant) return;
  const name = getTenantName();
  if (name) {
    const tenant = conf.multiTenant[name];
    if (tenant && !tenant.authProvider) {
      tenant.authProvider = "auth0";
    }
    return tenant;
  }
  return;
}

function defaultAuthInfo(conf = config()): AuthInfo | undefined {
  if (!conf) return;
  return {
    auth0Audience: conf.auth0Audience,
    auth0ClientId: conf.auth0ClientId,
    auth0Domain: conf.auth0Domain,
  };
}

function getTenantName(): string | null {
  const loggingInTenantName = getLogginInTenantName();
  if (loggingInTenantName) {
    return loggingInTenantName;
  }
  return window.localStorage.getItem(tenantKey);
}

export function logOutFromTenant() {
  window.localStorage.removeItem(tenantKey);
}

export function e2eAccessToken(): string | undefined {
  return window.FLOW_E2E_ACCESS_TOKEN;
}

export function logInToTenant() {
  const tenantName = getLogginInTenantName();
  const q = new URLSearchParams(window.location.search);
  if (tenantName && q.get("code")) {
    window.localStorage.setItem(tenantKey, tenantName);
  }
}

export function getSignInCallbackUrl() {
  const tenantName = getTenantName();
  if (tenantName) {
    // multi-tenant
    return `${window.location.origin}/auth/${tenantName}`;
  }
  return window.location.origin;
}
export function getAuthInfo(conf = config()): AuthInfo | undefined {
  return getMultitenantAuthInfo(conf) || defaultAuthInfo(conf);
}
