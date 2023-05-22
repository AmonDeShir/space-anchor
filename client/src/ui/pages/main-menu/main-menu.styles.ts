import { styled } from "solid-styled-components";
import image from "../../../assets/images/background_3.png";

export const BackgroundImage = styled.div`
  width: 100vw;
  height: 100vh;
  position: relative;
  background-image: url(${image});
  background-position: 0px 0px;
  background-size: cover;
  background-repeat: no-repeat;
`;

export const BackgroundGradient = styled.div`
  background: linear-gradient(78.95deg, rgba(0, 0, 0, 0.77) -0.74%, rgba(0, 0, 0, 0.2) 93.98%);
  backdrop-filter: blur(2.5px);
  width: 100vw;
  height: 100vh;
`;