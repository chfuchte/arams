import "@chfuchte/arams";

declare module "@chfuchte/arams" {
    /**
     * Checks the validity of the provided source code.
     * @param input Source code as string or array of strings (lines)
     * @returns true if the code is valid, otherwise an array of error messages with line numbers
     */
    function check(input: string | string[]):
        | true
        | Array<{
              line: number;
              message: string;
          }>;

    /**
     * Runs the provided source code with the given initial register values.
     * @param input Source code as string or array of strings (lines)
     * @param registers Initial register values as a Map<number, number>
     * @returns An object containing the final machine state (accumulator and registers) if successful,
     *          otherwise an array of error messages with line numbers (if compilation errors occur)
     * @throws Error if runtime errors occur
     */
    function run(
        input: string | string[],
        registers: Map<number, number>,
    ):
        | {
              accumulator: number;
              registers: Map<number, number>;
          }
        | Array<{
              line: number;
              message: string;
          }>;
}
