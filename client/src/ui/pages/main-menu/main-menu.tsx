import { createSignal } from "solid-js";
import { MainMenu } from "../../components/main-menu/main-menu";
import { BackgroundGradient, BackgroundImage } from "./main-menu.styles";

const MenuOptions = [
  "Continue",
  "New Game",
  "Load Game",
  "Multiplayer",
  "Mods",
  "Options",
  "Creators",
  "Exit",
];

export const MainMenuPage = () => {
  const [selected, setSelected] = createSignal<string | null>(null); 

  return (
    <BackgroundImage>
      <BackgroundGradient>
        <MainMenu options={MenuOptions} changeSelected={setSelected} show={selected() === null}  />
      </BackgroundGradient>
    </BackgroundImage>
  );
}