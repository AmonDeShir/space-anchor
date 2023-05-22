import { Route, Router, Routes } from "@solidjs/router";
import { FPS } from "./components/fps/fps";
import { render } from "solid-js/web";
import { MainMenuPage } from "./pages/main-menu/main-menu";

export const UI = () => {
  return (
    <Router>
      <FPS />

      <Routes>
        <Route path="/" component={MainMenuPage} />
      </Routes>
    </Router>
  );
};


export function createUI() {
  render(() => <UI />, document.querySelector("#ui"));
}