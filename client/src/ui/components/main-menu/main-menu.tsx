import { For, Show, createEffect, createSignal } from "solid-js";
import { Panel } from "../panel/panel";
import { PanelContainer, Container, LogoContainer, Logo, MenuItem } from "./main-menu.styles";
import { createAnimation } from "../../utils/create-animation";

type Props = {
  changeSelected: (menu: string) => void;
  show: boolean,
  options: string[]
}


export const MainMenu = (props: Props) => {
  const [display, setDisplay] = createSignal(props.show);

  let panel: HTMLDivElement; 
  let itemContainer: HTMLDivElement; 
  let logo: HTMLDivElement; 

  const panelEnter = createAnimation(tl => {
    tl
      .set(panel, { height: 50, translateX: -595 })
      .set(itemContainer, { scale: 0 })
      .to(panel, { translateX: 0, duration: 0.5 })
      .to(panel, { height: "100%", duration: 0.5 })
      .to(itemContainer, { scale: 1, duration: 0.40, delay: 0.02 }, "<")
  });

  const panelExit = createAnimation(tl => {
    tl
      .to(itemContainer, { scale: 0, duration: 0.40})
      .to(panel, { height: 50, duration: 0.5 }, "<")
      .to(panel, { translateX: -595, duration: 0.25 })
      .call(() => setDisplay(false));
  });

  const logoEnter = createAnimation(tl => {
    tl      
      .set(logo, { scale: 0 })
      .to(logo, { scale: 1, duration: 0.4, delay: 0.52 });
  });

  const logoExit = createAnimation(tl => {
    tl      
      .to(logo, { scale: 0, duration: 0.4 })
  });

  createEffect(() => {
    if (props.show) {
      setDisplay(true);
    }
  })

  createEffect(() => {
    if (props.show) {
      panelEnter();
      logoEnter();
    }
    else {
      panelExit();
      logoExit();
    }
  })

  return (
    <Show when={display()}>
      <Container>
        <PanelContainer ref={panel}>
          <Panel>
            <div ref={itemContainer}>
              <For each={props.options}>
                {(item) => (
                  <MenuItem onClick={() => props.changeSelected(item)}>{item}</MenuItem>
                )} 
              </For>
            </div>
          </Panel>
        </PanelContainer>

        <LogoContainer>
          <Logo ref={logo}>Space Anchor</Logo>
        </LogoContainer>
      </Container>
    </Show>
  );
}