import { styled } from "solid-styled-components";

export const Background = styled.div`
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.03);
  box-shadow: 2px 4px 4px rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(10px);
`;

export const Detal = styled.div`
  width: 100%;
  height: 25px;
  background: #FF7A00;
`;

export const Content = styled.div`
  width: 100%;
  height: calc(100% - 50px);
  display: flex;
  align-items: center;
  justify-content: center;
`;
