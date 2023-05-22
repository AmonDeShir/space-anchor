import { Store } from "../../../store/store";
import { Container } from "./fps.styles";

export const FPS = () => (
  <Container>
    {Store.debugInfo.fps.toFixed()} fps
  </Container>
);
