// Generated by cargo ts

export interface IrInspectResult {
   instruction: string;
   statements: (IrInspectResultSingle)[];
}

export interface IrInspectResultSingle {
   statement: string;
   data_accesses: (string)[];
   data_access_per_ir: (string)[];
}
