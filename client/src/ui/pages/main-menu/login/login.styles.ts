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

  //Animation
  transform: translateX(100vw);
`;


export const Text = styled.div`
  width: 100%;
  height: calc(100% - 60px);
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

export const Title = styled.div`
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

export const TextContainer = styled.div`
  width: 100%;
  height: calc(50% + 60px);
  display: flex;
  flex-direction: column;;
`;