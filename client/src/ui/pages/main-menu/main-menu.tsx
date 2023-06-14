import { createEffect, createResource, createSignal } from "solid-js";
import { MainMenu } from "./menu/menu";
import { BackgroundGradient, BackgroundImage } from "./main-menu.styles";
import { Login } from "./login/login";
import { Networking, createEventHandler } from "../../../network/network";
import { Registration } from "./registration/registration";
import { Store } from "../../../store/store";
import { useNavigate } from "@solidjs/router";

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

type RacesFile = {
  name: string,
  image: string,
  playable: boolean,
  description: {
    en: string,
    pl: string,
  }
}[];

export const MainMenuPage = () => {
  const [selected, setSelected] = createSignal<string | null>(null); 
  const [playerName, setPlayerName] = createSignal("");
  const [playerRace, setPlayerRace] = createSignal("");
  const [needsRegistration, setNeedsRegistration] = createSignal(false);
  const navigate = useNavigate();

  const [races] = createResource<RacesFile>(() => fetch("./data/races.json").then(data => data.json()));

  createEffect(() => {
    if (races() !== undefined && races().length > 0) {
      setPlayerRace(races()[0].name);
    }
  });

  const connect = () => {
    if (needsRegistration()) {
      Networking.send({ "Connect": playerName() })
      return;
    }

    if (playerName() !== "") {
      Networking.connect();
    }
  }

  createEventHandler("RequestConnect", () => {
    if (playerName() !== "") {
      Networking.send({ "Connect": playerName() })
    }
  })

  createEventHandler("RequestRegistration", () => {
    setNeedsRegistration(true);
    setSelected("Registration")
  })

  createEventHandler("ConnectionSuccess", () => {
    setNeedsRegistration(false);
    setSelected("Game");

    setTimeout(() => {
      Store.setOther("shouldRender", true);
        navigate("/game");
    }, 1000)
  })
  
  const register = () => {
    if (playerName() !== "" && playerRace() !== "") {
      Networking.send({ "Register": { name: playerName(), race: playerRace() } })
    }
  }

  window.addEventListener("keydown", (ev) => {
    // Shift+R
    if (ev.shiftKey && ev.key === 'R') {
      Networking.on("RequestConnect", () => {
        Networking.send({ "Connect": "Amon" })
      })
  
      Networking.on("RequestRegistration", () => {
        Networking.send({ "Register": { name: "Amon", race: "solis" } })
      })
  
      Networking.on("ConnectionSuccess", () => {
        if (window.location.pathname == '/') {
          return;
        }
        
        setTimeout(() => {
          Store.setOther("shouldRender", true);
          window.location.href = '/game';
        }, 1000)
      })
  
      setTimeout(() => Networking.connect(), 200);
    }
  });

  return (
    <BackgroundImage>
      <BackgroundGradient>
        <MainMenu options={MenuOptions} changeSelected={setSelected} show={selected() === null}  />
        <Login 
          show={selected() === "Multiplayer"}
          name={playerName()}
          onNameChange={setPlayerName}
          onBack={() => {setSelected(null); setNeedsRegistration(false); Networking.disconnect(); }}
          onConnect={connect}
          changeSelected={setSelected} 
        />

        <Registration 
          show={selected() === "Registration"}
          race={playerRace()}
          onRaceChange={setPlayerRace}
          onBack={() => setSelected("Multiplayer")}
          onRegister={register}
          changeSelected={setSelected} 
          races={races()}
        />
      </BackgroundGradient>
    </BackgroundImage>
  );
}