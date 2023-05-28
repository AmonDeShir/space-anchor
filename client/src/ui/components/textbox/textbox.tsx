import { styled } from "solid-styled-components";

export const Textbox = styled.input`
  height: 120px;
  background-color: transparent;
  border: 8px solid var(--main);
  border-width: 8px 0 8px 0;
  box-sizing: border-box;
  text-align: center;
  color: white;
  font-size: 40px;
  transition: border-width 0.1s, font-size 0.1s;
  font-family: "Fervojo";

  &:focus {
    outline: none;
  }

  &:hover {
    border-width: 10px 0 10px 0;
    font-size: 38px;
  }
`;