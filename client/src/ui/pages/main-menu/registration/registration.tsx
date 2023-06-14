import { Show, createEffect, createMemo, createSignal } from "solid-js";
import { Panel } from "../../../components/panel/panel";
import { PanelContainer, Container, TitleContainer, TextContainer, Text, PageTitle, Buttons, Button, PanelContent, PanelContainerBig, PanelContainerSmall, Image, ImageContainer, SmallPanelContent } from "./registration.styles";
import { createAnimation, setRefFor, setRefForAdv } from "../../../utils/create-animation";
import { Title } from "../../../components/title/title";
import { Select } from "../../../components/select/select";

type Props = {
  changeSelected: (menu: string) => void;
  show: boolean;
  race: string,
  onBack: () => void;
  onRaceChange: (name: string) => void;
  onRegister: () => void;
  races: Race[],
}

type PanelRef = { panel: HTMLElement, container: HTMLElement };

type Race = {
  name: string,
  image: string,
  playable: boolean,
  description: {
    en: string,
    pl: string,
  }
};

export const Registration = (props: Props) => {
  const [display, setDisplay] = createSignal(props.show);

  const createEnterAnimation = (top: boolean) => createAnimation<PanelRef>((tl, { panel, container }) => {
    tl
      .set(panel, { height: 50, translateY: top ? "-100vh" : "100vh"})
      .set(container, { scale: 0 })
      .to(panel, { translateY: "0vh", duration: 0.5, delay: 0.5 })
      .to(panel, { height: "100%", duration: 0.5 })
      .to(container, { scale: 1, duration: 0.40, delay: 0.02 }, "<")
  }, { panel: null, container: null });

  const createExitAnimation = (top: boolean) => createAnimation<PanelRef>((tl, { panel, container }) => {
    tl
      .set(panel, { height: "100%", translateY: "0vh"})
      .set(container, { scale: 1 })
      .to(container, { scale: 0, duration: 0.40})
      .to(panel, { height: 50, duration: 0.5 }, "<")
      .to(panel, { translateY: top ? "-100vh" : "100vh", duration: 1.0 })
      .call(() => { setDisplay(false) });
  }, { panel: null, container: null });


  const panelEnter = createEnterAnimation(false);
  const smallPanelEnter = createEnterAnimation(true);

  const panelExit = createExitAnimation(false);
  const smallPanelExit = createExitAnimation(true);

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
    console.log("update 1 ", props.show, Date.now())

    if (props.show) {
      setDisplay(true);

    }
  })

  createEffect(() => {
    console.log("update 2", display(), Date.now())
  });

  createEffect(() => {
    if (props.show) {
      panelEnter.start();
      smallPanelEnter.start();
      titleEnter.start();
    }
    else {
      panelExit.start();
      smallPanelExit.start();
      titleExit.start();
    }
  })

  const race = createMemo(() => props.races?.find(race => race.name === props.race));

  return (
    <Show when={display()}>
      <Container>
        <TitleContainer ref={setRefFor(titleEnter, titleExit)}>
          <PageTitle>New Character</PageTitle>
        </TitleContainer>

        <PanelContainer>
          <PanelContainerBig ref={setRefForAdv((ref, old) => ({...old, panel: ref}), panelEnter, panelExit)}>
            <Panel>
                <PanelContent ref={setRefForAdv((ref, old) => ({...old, container: ref}), panelEnter, panelExit)}>
                  <TextContainer>
                    <Text>Select your race</Text>
                    <Select items={props.races.filter(race => race.playable).map(race => race.name)} onChange={props.onRaceChange} value={props.race} />
                  </TextContainer>

                  <Buttons>
                    <Button onClick={props.onBack}>Back</Button>
                    <Button onClick={props.onRegister}>Play</Button>
                  </Buttons>
                </PanelContent>
              </Panel>
          </PanelContainerBig>

          <PanelContainerSmall ref={setRefForAdv((ref, old) => ({...old, panel: ref}), smallPanelEnter, smallPanelExit)}>
            <Panel>
                <SmallPanelContent ref={setRefForAdv((ref, old) => ({...old, container: ref}), smallPanelEnter, smallPanelExit)}>
                  <ImageContainer>
                    <Image src={race()?.image ?? ""} />
                  </ImageContainer>
                  <Title width="100%">{race()?.name ?? props.race}</Title>
                  <div>
                    {race()?.description.en ?? "" }
                  </div>
                </SmallPanelContent>
              </Panel>
          </PanelContainerSmall>
        </PanelContainer>
      </Container>
    </Show>
  );
}