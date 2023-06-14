import { createSignal } from "solid-js";
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

const Races = [
  "Mortemii",
  "Krushaz",
  "Solis",
  "Aquaformae",
  "Drongarianii"
];

const Texts = {
  "Mortemii": "Najbardziej zaawansowana cywilizacja. Nie wiadomo, z jakiej galaktyki pochodzą, sami nie pamiętają. Według samych siebie istnieją od zawsze i zawsze istnieć będą — rasa post transcendentna.",
  "Krushaz": "Nazywają siebie zielone hopaki, są sztuczną rasą, stworzoną do prowadzenia wojen.",
  "Solis": "Nazywają siebie terrans, rasa napływowa z sąsiadującej drogi mlecznej, pochodzą z układu słonecznego, ich planeta macierzysta to ziemia.",
  "Aquaformae": "Jedna z dwóch inteligentnych nienapływowych ras zamieszkująca galaktykę Anchor,  pochodzą z wodnej planety. Ich statki są wypełnione wodą, urodzeni kupcy. Mają najlepsze statki transportowe.",
  "Drongarianii": "Cywilizacja Drongarian opiera się mentalności roju gdzie każda istota jest połączona, często w dosłownym znaczeniu, z resztą kolonii. Każda istota w kolonii Drongarian jest dostosowana do wykonywania jednego zadania mającego zaspokoić potrzeby innych części Kolonii.",
}


export const MainMenuPage = () => {
  const [selected, setSelected] = createSignal<string | null>(null); 
  const [playerName, setPlayerName] = createSignal("");
  const [playerRace, setPlayerRace] = createSignal(Races[0]);
  const [needsRegistration, setNeedsRegistration] = createSignal(false);
  const navigate = useNavigate();

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
          texts={Texts}
          races={Races}
        />
      </BackgroundGradient>
    </BackgroundImage>
  );
}