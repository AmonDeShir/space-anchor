import { styled } from "solid-styled-components";

export const Container = styled.div`
  position: fixed;
  left: 0px;
  top: 0px;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

export const PanelContainer = styled.div`
  width: calc(100% - 170px);
  height: calc(100% - 350px);
  max-height: 970px;
  max-width: 2000px;
  position: relative;
  z-index: 2;
  justify-content: space-between;
  display: flex;
`;

export const PanelContainerSmall = styled.div`
  width: calc(30%);
  height: 100%;
  max-height: 970px;
  max-width: 505px;
  position: relative;
  z-index: 2;

  //Animation
  transform: translateY(-100vh);
`;

export const PanelContainerBig = styled.div`
  width: calc(100% - min(520px, 30% + 15px));
  height: 100%;
  max-height: 970px;
  max-width: 2000px;
  position: relative;
  z-index: 2;

  //Animation
  transform: translateY(100vh);
`;



export const Text = styled.div`
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 40px;
  text-align: center;
`;

export const TitleContainer = styled.div`
  width: 100%;
  height: 300px;
  display: flex;
  justify-content: center;
  align-items: center;
`;

export const PageTitle = styled.div`
  color: var(--main);
  font-size: 150px;
  font-family: 'New Rocker';
  text-align: center;
`;

export const Buttons = styled.div`
  width: 100%;
  display: flex;
  justify-content: space-between;
`;

export const Button = styled.div`
  font-size: 40px;
  margin: 30px 0 30px 0;
  text-align: center;
  color: var(--text);
  font-family: 'Radiotechnika';
  transition: color 0.25s;
  cursor: pointer;

  &:hover {
    color: var(--main);
  }
`;

export const PanelContent = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 0 40px 0 40px;
`;

export const SmallPanelContent = styled(PanelContent)`
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  flex-direction: column;
`;

export const TextContainer = styled.div`
  width: 100%;
  height: calc(100% - 100px);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
`;


export const ImageContainer = styled.div`
  display: grid; 
  aspect-ratio: 1;
  max-width: 300px; 
  width: 100%; 
  overflow: hidden;
  position: relative;

  &::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    box-shadow: inset 0px 4px 4px 2px rgba(0, 0, 0, 0.5);
  }
`;


export const Image = styled.img`
  object-fit: cover;
  width: 100%;
  height: 100%;
`;