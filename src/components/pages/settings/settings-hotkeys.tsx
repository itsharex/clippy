import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { Component, For } from "solid-js";
import { TextBlock } from "../../elements/text-block";
import { Shortcut } from "../../utils/shortcut";
import { HotkeyStore } from "../../../store/hotkey-store";

interface SettingsHotkeysProps {}

export const SettingsHotkeys: Component<SettingsHotkeysProps> = ({}) => {
  const { hotkeys } = HotkeyStore;

  return (
    <>
      <TextBlock Icon={RiDeviceKeyboardFill} title="Change your Hotkeys">
        <div class="h-64 overflow-auto px-5">
          <For each={hotkeys()}>
            {(hotkey, index) => {
              return (
                <>
                  <div class="flex items-center px-0.5 py-4">
                    <Shortcut hotkey={hotkey} />
                  </div>
                  {hotkeys().length !== index() + 1 && <hr class="border-zinc-700" />}
                </>
              );
            }}
          </For>
        </div>
      </TextBlock>
    </>
  );
};
