// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { GeneratedExpressionMode } from "./GeneratedExpressionMode";
import type { ReferentialAction } from "./ReferentialAction";

export type ColumnOption = "Null" | "NotNull" | { "Default": string } | { "Unique": { is_primary: boolean, } } | { "ForeignKey": { foreign_table: string, referred_columns: Array<string>, on_delete: ReferentialAction | null, on_update: ReferentialAction | null, } } | { "Check": string } | { "OnUpdate": string } | { "Generated": { expr: string, mode: GeneratedExpressionMode | null, } };
