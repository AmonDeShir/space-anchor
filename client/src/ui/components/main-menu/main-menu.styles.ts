import { styled } from "solid-styled-components";

export const Container = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  position: relative;
`;

export const PanelContainer = styled.div`
  width: 100%;
  height: 100%;
  max-height: 970px;
  max-width: 595px;
  position: relative;
  z-index: 2;
`;


export const LogoContainer = styled.div`
  position: absolute;
  width: calc(100% - 595px);
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  right: 0px;
`;

export const Logo = styled.div`
  height: 440px;
  width: 650px;
  color: var(--main);
  font-size: 200px;
  font-family: 'New Rocker';
  text-align: center;
`;

export const MenuItem = styled.div`
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