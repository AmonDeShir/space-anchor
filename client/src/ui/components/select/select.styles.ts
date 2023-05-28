import { css, styled } from "solid-styled-components";

export const Container = styled.div`
  padding: 10px 5px;
  background-color: transparent;
  border: 8px solid var(--main);
  border-width: 8px 0 8px 0;
  box-sizing: border-box;
  text-align: center;
  font-size: 40px;
  font-family: "Fervojo";
  width: 350px;
`;

export const Value = styled.div<{size: boolean}>`
  transition: color 0.25s, transform 0.25s;
  color: ${props => props.size ? `var(--main)` : `var(--text)`};
  transform: scale(1.0);
  cursor: pointer;
  user-select: none;

  &:hover {
    transform: scale(0.9);
  }
`;