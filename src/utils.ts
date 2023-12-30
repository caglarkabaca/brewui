import { Ref } from "vue";
import { invoke } from "@tauri-apps/api";

import { _Metadata } from "./components/List.vue";
import { Metadata } from "./bindings/Metadata";
import { Root } from "./bindings/Root.ts";

enum Type {
    Formula,
    Cask
}

export async function Load_Installed(formulas: Ref<string[]>, casks: Ref<string[]>) {
    interface Response {
        name: string[];
        count: number;
    }
    let load_formulas = async () => {
        await invoke<Response>("get_installed_formulas", {}).then((r) => {
            r.name.forEach(x => {
                formulas.value.push(x);
            });
        }).catch((e) => console.error(e));
    }
    let load_casks = async () => {
        await invoke<Response>("get_installed_casks", {}).then((r) => {
            r.name.forEach(x => {
                casks.value.push(x);
            });
        }).catch((e) => console.error(e));
    }
    await load_formulas();
    await load_casks();
}

export async function Load_Metadata(installed_formulas: string[], installed_casks: string[], metadatas: _Metadata[]) {
    await invoke<Array<Metadata>>("get_all_formulas", {}).then((r) => {
        r.forEach((metadata) => {
            if (metadata.name != null) {
                let installed = installed_formulas.concat(installed_casks).filter((p) => p.split(" ")[0] == metadata.name!).length > 0;
                let deprecated = installed_formulas.concat(installed_casks).filter((p) => installed && p.split(" ")[1] == metadata.versions!.stable).length > 0;
                metadatas.push({
                    data: metadata,
                    installed: installed,
                    deprecated: !deprecated,
                    type: Type.Formula
                });
            }
        })
    }).catch((e) => console.error(e));
    await invoke<Array<Metadata>>("get_all_casks", {}).then((r) => {
        r.forEach((metadata) => {
            if (metadata.name != null) {
                let installed = installed_formulas.concat(installed_casks).filter((p) => p.split(" ")[0] == metadata.name!).length > 0;
                let deprecated = installed_formulas.concat(installed_casks).filter((p) => installed && p.split(" ")[1] == metadata.versions!.stable).length > 0;
                metadatas.push({
                    data: metadata,
                    installed: installed,
                    deprecated: !deprecated,
                    type: Type.Cask
                });
                if (metadata.name === 'raycast') {
                    console.log(metadata);
                }
            }
        })
    }).catch((e) => console.error(e));
    metadatas.sort((a, b) => {
        if (a.data.name! > b.data.name!) {
            return 1;
        } else if (a.data.name! < b.data.name!) {
            return -1;
        }
        return 0;
    })
}

export async function Load_Info(model: { name: string; data: Ref<Root | undefined>, isCask: Ref<boolean | undefined> }) {
    let is_cask = false;
    await invoke<Root>("get_info_formula", { name: model.name }).then((e) => model.data.value = e).catch((e) => is_cask = true);
    if (is_cask) {
        await invoke<Root>("get_info_cask", { name: model.name }).then((e) => model.data.value = e).catch((e) => console.error(e));
    }
    model.isCask.value = is_cask;
}