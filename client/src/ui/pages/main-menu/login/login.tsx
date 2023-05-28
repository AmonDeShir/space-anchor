import { Show, createEffect, createSignal } from "solid-js";
import { Panel } from "../../../components/panel/panel";
import { PanelContainer, Container, TitleContainer, TextContainer, Text, Title, Buttons, Button, PanelContent } from "./login.styles";
import { Textbox } from "../../../components/textbox/textbox";
import { createAnimation, setRefFor, setRefForAdv } from "../../../utils/create-animation";

type Props = {
  changeSelected: (menu: string) => void;
  show: boolean;
  name: string,
  onBack: () => void;
  onNameChange: (name: string) => void;
  onConnect: () => void;
}

type PanelRef = { panel: HTMLElement, container: HTMLElement };

export const Login = (props: Props) => {
  const [display, setDisplay] = createSignal(props.show);

  const panelEnter = createAnimation<PanelRef>((tl, { panel, container }) => {
    tl
      .set(panel, { height: 50, translateX: "100vw"})
      .set(container, { scale: 0 })
      .to(panel, { translateX: "0vw", duration: 0.5, delay: 0.5 })
      .to(panel, { height: "calc(100% - 350px)", duration: 0.5 })
      .to(container, { scale: 1, duration: 0.40, delay: 0.02 }, "<")
  }, { panel: null, container: null });

  const panelExit = createAnimation<PanelRef>((tl, { panel, container }) => {
    tl
      .set(panel, { height: "calc(100% - 350px)", translateX: "0vw"})
      .set(container, { scale: 1 })
      .to(container, { scale: 0, duration: 0.40})
      .to(panel, { height: 50, duration: 0.5 }, "<")
      .to(panel, { translateX: "100vw", duration: 0.25 })
      .call(() => { setDisplay(false) });
  }, { panel: null, container: null });

  const titleEnter = createAnimation((tl, title) => {
    tl      
      .set(title, { scale: 0 })
      .to(title, { scale: 1, duration: 0.4, delay: 1.02 });
  });

  const titleExit = createAnimation((tl, title) => {
    tl      
      .to(title, { scale: 0, duration: 0.4 })
  });

  createEffect(() => {
    if (props.show) {
      setDisplay(true);

    }
  })

  createEffect(() => {
    if (props.show) {
      panelEnter.start();
      titleEnter.start();
    }
    else {
      panelExit.start();
      titleExit.start();
    }
  })


  return (
    <Show when={display()}>
      <Container>
        <TitleContainer ref={setRefFor(titleEnter, titleExit)}>
          <Title>Multiplayer</Title>
        </TitleContainer>

        <PanelContainer ref={setRefForAdv((ref, old) => ({...old, panel: ref}), panelEnter, panelExit)}>
          <Panel>
            <PanelContent ref={setRefForAdv((ref, old) => ({...old, container: ref}), panelEnter, panelExit)}>
              <TextContainer>
                <Text>Enter the name of your <br/> character</Text>
                <Textbox type="textbox" value={props.name} placeholder="Character Name" oninput={(e) => props.onNameChange(e.target.value)} />
              </TextContainer>

              <Buttons>
                <Button onClick={props.onBack}>Back</Button>
                <Button onClick={props.onConnect}>Connect</Button>
              </Buttons>
            </PanelContent>
          </Panel>
        </PanelContainer>
      </Container>
    </Show>
  );
}