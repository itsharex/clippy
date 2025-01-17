import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Component } from "solid-js";

type SwitchProps = {
  checked?: boolean;
  onClick?: () => Promise<void> | undefined;
  onChange: (val: boolean) => Promise<void> | undefined;
};

export const Toggle: Component<SwitchProps> = (props) => {
  const handleChange = () => {
    props.onChange(!props.checked);
  };

  return (
    <label class="relative inline-flex cursor-pointer items-center" onClick={props.onClick}>
      <input
        type="checkbox"
        checked={props.checked}
        onChange={handleChange}
        class="peer sr-only"
        disabled={!!props.onClick}
      />
      <div
        class={`peer relative h-4 w-11 rounded-full bg-gray-200 after:absolute after:start-[2px] after:top-0 after:h-4 after:w-4 after:rounded-full after:border after:border-transparent after:bg-white after:transition-all after:content-[''] peer-checked:bg-indigo-600 peer-checked:after:translate-x-[150%] peer-checked:after:border-transparent peer-focus:outline-none dark:border-gray-600 rtl:peer-checked:after:-translate-x-full ${
          props.checked ? "dark:bg-gray-700" : "dark:bg-red-600"
        } after:z-[40] dark:bg-opacity-20 after:dark:bg-zinc-700`}
      >
        <div class="absolute inset-0 z-[50] flex items-center justify-between px-1">
          {props.checked ? <FiCheck class="ml-auto text-sm text-white" /> : <VsClose class="text-white" />}
        </div>
      </div>
    </label>
  );
};
