import ReactDOM from "react-dom/client";
import { HashRouter as Router, Routes, Route } from "react-router-dom";

import Layout from "./components/Layout";

import Dashboard from "./views/Dashboard";
import DiffViewer from "./views/DiffViewer";
import About from "./views/About";
import Changelog from "./views/Changelog";
import Utilities from "./views/Utilities";
import TaskManager from "./views/TaskManager";
import TestPage from "./views/testpage";

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Could not find root element to mount to");
}

const root = ReactDOM.createRoot(rootElement);
root.render(
  <Router>
    <Layout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/diffs" element={<DiffViewer />} />
        <Route path="/taskmanager" element={<TaskManager />} />
        <Route path="/utilities" element={<Utilities />} />
        <Route path="/changelog" element={<Changelog />} />
        <Route path="/about" element={<About />} />
        <Route path="/test" element={<TestPage />} />
      </Routes>
    </Layout>
  </Router>,
);
