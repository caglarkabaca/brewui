// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Analytic } from "./Analytic";
import type { Bottle } from "./Bottle";
import type { Versions } from "./Versions";

export interface Root { name: string | null, full_name: string | null, tap: string | null, aliases: Array<string> | null, desc: string | null, license: string | null, homepage: string | null, versions: Versions | null, bottle: Record<string, Bottle> | null, build_dependencies: Array<string> | null, dependencies: Array<string> | null, analytics: Record<string, Analytic> | null, }