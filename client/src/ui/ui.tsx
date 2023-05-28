import { Route, Router, Routes } from "@solidjs/router";
import { FPS } from "./components/fps/fps";
import { render } from "solid-js/web";
import { MainMenuPage } from "./pages/main-menu/main-menu";
import 'solid-devtools'
import { InGameUIPage } from "./pages/in-game-ui/in-game-ui";

export const UI = () => {
  return (
    <Router>
      <FPS />

      <Routes>
        <Route path="/" component={MainMenuPage} />
        <Route path="/game" component={InGameUIPage} />
      </Routes>
    </Router>
  );
};


export function createUI() {
  render(() => <UI />, document.querySelector("#ui"));
}