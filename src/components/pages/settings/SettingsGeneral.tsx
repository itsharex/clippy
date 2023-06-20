import { BsBellFill } from "solid-icons/bs";
import { FaRegularKeyboard, FaRegularMoon } from "solid-icons/fa";
import { TiCogOutline } from "solid-icons/ti";
import { VsRocket } from "solid-icons/vs";
import { Component } from "solid-js";
import { Hotkey } from "../../../@types";
import SettingsStore from "../../../store/SettingsStore";
import SwitchField from "../../elements/SwitchField";
import { TextBlock } from "../../elements/TextBlock";
import { DarkMode } from "../../utils/DarkMode";
import { Shortcut } from "../../utils/Shortcut";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  const { hotkeys, settings, updateSettings } = SettingsStore;
  const hotkey = hotkeys().find(
    (key) => key.event === "window_display_toggle"
  ) as Hotkey;

  return (
    <>
      <TextBlock Icon={FaRegularKeyboard} title="Keyboard shortcut">
        <div class="mb-2 flex items-center space-x-2 px-5 pb-2.5">
          <Shortcut hotkey={hotkey} />
        </div>
      </TextBlock>

      <TextBlock Icon={TiCogOutline} title="System">
        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <VsRocket />
            <h6 class="text-sm">Start Clippy on system startup.</h6>
          </div>
          <div>
            <SwitchField
              checked={settings()?.startup || false}
              onChange={(check: boolean) =>
                updateSettings({ ...settings()!, startup: check })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <BsBellFill />
            <h6 class="text-sm">Show desktop notifications.</h6>
          </div>
          <div>
            <SwitchField
              checked={settings()?.notification || false}
              onChange={(check: boolean) =>
                updateSettings({ ...settings()!, notification: check })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <FaRegularMoon />
            <h6 class="text-sm">Switch Theme.</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>
      </TextBlock>
    </>
  );
};