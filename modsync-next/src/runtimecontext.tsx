import React from "react";
import { RuntimeInfo } from "./types";

export const RuntimeContext = React.createContext<RuntimeInfo>({
  title: "",
  version: "",
  buildinfo: "",
  release_info: [],
  has_serverdat: false,
  has_options: false,
  has_hcml: false,
  has_pclce: false,
  has_configpack: false,
});
