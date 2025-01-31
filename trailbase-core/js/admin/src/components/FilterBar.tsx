import { Button } from "@/components/ui/button";
import { TextField, TextFieldInput } from "@/components/ui/text-field";

export function FilterBar(props: {
  example?: string;
  placeholder?: string;
  initial?: string;
  onSubmit: (filter: string) => void;
}) {
  let ref: HTMLInputElement | undefined;
  const onSubmit = () => {
    const value = ref?.value;
    console.debug("set filter: ", value);
    if (value !== undefined) {
      props.onSubmit(value);
    }
  };

  return (
    <div class="w-full flex flex-col">
      <form
        class="flex w-full items-center gap-2"
        onSubmit={onSubmit}
        action="javascript:void(0);"
      >
        <TextField class="w-full">
          <TextFieldInput
            ref={ref}
            value={props.initial}
            type="text"
            placeholder={props.placeholder ?? "filter"}
          />
        </TextField>

        <Button type="button">Filter</Button>
      </form>

      {props.example && <span class="text-sm mt-1 ml-2">{props.example}</span>}
    </div>
  );
}
