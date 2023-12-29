import { Ref } from "vue";
import { invoke } from "@tauri-apps/api";

import { _Metadata } from "./components/List.vue";
import { Metadata } from "./bindings/Metadata";

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
}