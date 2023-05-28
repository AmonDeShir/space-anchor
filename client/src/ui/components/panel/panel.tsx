import { ParentProps } from "solid-js";
import { Background, Content, Detal } from "./panel.styles";

type Props = ParentProps<{}>;

export const Panel = (props: Props) => {
  return (
    <Background>
      <Detal />
      <Content>
        {props.children}
      </Content>
      <Detal />
    </Background>
  );
}