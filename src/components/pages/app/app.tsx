import { BsHddFill } from "solid-icons/bs";
import { FiGlobe } from "solid-icons/fi";
import { Show } from "solid-js";
import { AppSidebar } from "../../navigation/app-sidebar";
import { ClipboardHistory } from "./clipboard-history";
import { RecentClipboards } from "./recent-clipboards";
import { StarredClipboards } from "./starred-clipboards";
import { ViewMore } from "./view-more";
import { AppStore } from "../../../store/app-store";
import { SettingsStore } from "../../../store/settings-store";

function App() {
  const { settings } = SettingsStore;
  const { getCurrentTab } = AppStore;

  return (
    <div class="flex h-full w-full overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <div class="flex w-12 flex-col items-center space-y-3 bg-gray-200 pt-2 dark:bg-dark-light">
        <AppSidebar />
      </div>
      <div class="flex h-screen min-w-0 flex-1 flex-col">
        <div class="flex w-full justify-between px-2 py-1">
          <p class="select-none bg-gray-50 text-xs font-semibold text-gray-500 dark:bg-dark-dark dark:text-white">
            {getCurrentTab()?.name?.toUpperCase()}
          </p>
          <Show when={settings()?.synchronize} fallback={<BsHddFill title="offline" />}>
            <FiGlobe title="online" />
          </Show>
        </div>

        <Show when={getCurrentTab()?.name === "Recent Clipboards"}>
          <RecentClipboards />
        </Show>

        <Show when={getCurrentTab()?.name === "Starred Clipboards"}>
          <StarredClipboards />
        </Show>

        <Show when={getCurrentTab()?.name === "History"}>
          <ClipboardHistory />
        </Show>

        <Show when={getCurrentTab()?.name === "View more"}>
          <ViewMore />
        </Show>
      </div>
    </div>
  );
}

export default App;
