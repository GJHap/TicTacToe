import Cell from "./Cell";

export default function ({
    row,
    rowIdx,
    onCellClicked,
}: {
    row: string[];
    rowIdx: number;
    onCellClicked: (rowIdx: number, colIdx: number) => void;
}) {
    return (
        <div className="flex flex-row h-full w-full">
            {row.map((cell, colIdx) => (
                <Cell
                    key={`${rowIdx},${colIdx}`}
                    value={cell}
                    onClick={() => onCellClicked(rowIdx, colIdx)}
                />
            ))}
        </div>
    );
}
