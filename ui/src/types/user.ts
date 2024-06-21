import { ApiResponse } from "./api";

export type GetMe = {
  me: Me | undefined;
  isLoading: boolean;
} & ApiResponse;

export type Me = {
  myWorkspaceId: string;
  lang?: string;
  theme?: string;
} & User;

export type User = {
  id: string;
  name: string;
  email: string;
};