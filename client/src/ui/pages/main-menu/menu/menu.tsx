import { For, Show, createEffect, createSignal } from "solid-js";
import { Panel } from "../../../components/panel/panel";
import { PanelContainer, Container, LogoContainer, Logo, MenuItem } from "./menu.styles";
import { createAnimation, setRefFor, setRefForAdv } from "../../../utils/create-animation";

type Props = {
  changeSelected: (menu: string) => void;
  show: boolean,
  options: string[]
}

type PanelRef = { panel: HTMLElement, container: HTMLElement };

export const MainMenu = (props: Props) => {
  const [display, setDisplay] = createSignal(props.show);

  const panelEnter = createAnimation<PanelRef>((tl, { panel, container }) => {

    tl
      .set(panel, { height: 50, translateX: -595})
      .set(container, { scale: 0 })
      .to(panel, { translateX: 0, duration: 0.5, delay: 0.5 })
      .to(panel, { height: "100%", duration: 0.5 })
      .to(container, { scale: 1, duration: 0.40, delay: 0.02 }, "<")
  }, { panel: null, container: null });

  const panelExit = createAnimation<PanelRef>((tl, { panel, container }) => {

    tl
      .to(container, { scale: 0, duration: 0.40})
      .to(panel, { height: 50, duration: 0.5 }, "<")
      .to(panel, { translateX: -595, duration: 0.25 })
      .call(() => { setDisplay(false) });
  }, { panel: null, container: null });

  const logoEnter = createAnimation((tl, logo) => {

    tl      
      .set(logo, { scale: 0 })
      .to(logo, { scale: 1, duration: 0.4, delay: 1.02 });
  });

  const logoExit = createAnimation((tl, logo) => {

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
      panelEnter.start();
      logoEnter.start();
    }
    else {
      panelExit.start();
      logoExit.start();
    }
  })

  return (
    <Show when={display()}>
      <Container>
        <PanelContainer ref={setRefForAdv((ref, old) => ({...old, panel: ref}), panelEnter, panelExit)}>
          <Panel>
            <div ref={setRefForAdv((ref, old) => ({...old, container: ref}), panelEnter, panelExit)}>
              <For each={props.options}>
                {(item) => (
                  <MenuItem onClick={() => props.changeSelected(item)}>{item}</MenuItem>
                )} 
              </For>
            </div>
          </Panel>
        </PanelContainer>

        <LogoContainer>
          <Logo ref={setRefFor(logoEnter, logoExit)}>Space Anchor</Logo>
        </LogoContainer>
      </Container>
    </Show>
  );
}