import { styled } from "solid-styled-components";

export const Title = styled.div<{ width?: string }>`
  height: 64px;
  background-color: transparent;
  border: 8px solid var(--main);
  border-width: 8px 0 8px 0;
  box-sizing: border-box;
  text-align: center;
  color: white;
  font-size: 25px;
  transition: border-width 0.1s, font-size 0.1s;
  font-family: "Fervojo";
  display: flex;
  justify-content: center;
  align-items: center;

  ${props => props.width ? "width: " + props.width + ";" : ''}
`;