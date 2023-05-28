import { styled } from "solid-styled-components";
import image from "../../../assets/images/background_6.png";

export const BackgroundImage = styled.div`
  width: 100vw;
  height: 100vh;
  position: relative;
  background-position: center;
  background-image: url(${image});
  background-size: cover;
  background-repeat: no-repeat;
`;

export const BackgroundGradient = styled.div`
  width: 100vw;
  height: 100vh;
`;