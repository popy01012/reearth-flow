import { createLazyFileRoute } from "@tanstack/react-router";

import { LoadingPage } from "@flow/pages";

export const Route = createLazyFileRoute("/")({
  component: Index,
});

function Index() {
  return <LoadingPage />;
}
