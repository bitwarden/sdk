export type Input = [string, InputType];

export type InputType = {
  resolved_path?: {
    name: string;
    args: {
      angle_bracketed: {
        args: any[];
      };
    };
  };
};
