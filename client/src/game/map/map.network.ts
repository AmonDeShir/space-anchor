import { Networking } from "../../network/network";
import { Store } from "../../store/store";
import { Admiral, AdmiralPacket, AdmiralsPosition, spawnAdmiral } from "../admiral/admiral";
import { BabylonWorld } from "../babylon/babylon";
import { createPlayer } from "../player/player";
import { Player } from "../player/player";
import { SingleMapState } from "./map";

/** Listens for the ConnectionSuccess message and fetches the PlayerAdmiralId and current map state from the server. */
export function fetchMapStateAfterLogin(admirals: { [key: number]: Admiral }) {
  Networking.on("ConnectionSuccess", () => {
    setTimeout(() => Networking.send({ "GetPlayerAdmiralId": [] }), 1000);
  });

  Networking.on("PlayerAdmiralId", (id) => {
    Store.setPlayer("id", id);
  
    setTimeout(() => {
      if (Object.keys(admirals).length === 0) {
        Networking.send({ GetAllAdmiralsInSign: [] });
      }
    }, 1000);
  });  
}

/** Listens for AdmiralAppears and AdmiralDisappears messages and performs actions based on these events. */
export function handlePlayerSightRadiusUpdate(world: BabylonWorld, admirals: { [key: number]: Admiral }, setPlayer: (setter: (player: Player) => Player) => void) {
  Networking.on<AdmiralPacket>("AdmiralAppears", async (admiral) => {
    if (!admirals[admiral.id]) {
      admirals[admiral.id] = await (spawnAdmiral(admiral, world));
    }
  
    admirals[admiral.id].state.set("appears");
    
    setPlayer((player) => {
      if (player == null) {
        player = createPlayer(admirals[Store.player.id]);
      }

      return player;
    });
  });
  
  Networking.on<number>("AdmiralDisappears", async (id) => {
    admirals[id]?.state!.set("disappear");
  });
  
}

/** Listens for the PositionOfAdmiralsInSign message and adds the received state to the map states. */
export function handleMapStateUpdate(states: SingleMapState[]) {
  Networking.on<[AdmiralsPosition[], number]>("PositionOfAdmiralsInSign", (state) => {
    states.push({
      admirals: state[0].map((admiral) => ({ id: admiral[0], pos: admiral[1], rot: admiral[2] })),
      time: state[1]
    });
  });
}
