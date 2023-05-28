import { For } from "solid-js";
import { Container, Value } from "./select.styles";

type Props = {
  items: string[],
  value: string,
  onChange: (value: string) => void,
}

export const Select = (props: Props) => {
  return (
    <Container>
      <For each={props.items}>
        {(item) => (
          <Value size={item === props.value} onClick={() => props.onChange(item)}>
            {item}
          </Value>
        )}
      </For>
    </Container>
  );
};