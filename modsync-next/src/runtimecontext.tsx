import React from "react";
import { RuntimeInfo } from "./types";

export const RuntimeContext = React.createContext<RuntimeInfo>({ title: "", version: "", buildinfo: "", is_update_available: false, release_info: [] });