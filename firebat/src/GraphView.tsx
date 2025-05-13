import React, { useMemo } from 'react';
import ReactFlow, { Node, Edge, Controls, Background } from 'react-flow-renderer';
import 'react-flow-renderer/dist/style.css';
import { IrInspectResult } from './bindings';

interface GraphViewProps {
  data: IrInspectResult[];
}

const GraphView: React.FC<GraphViewProps> = ({ data }) => {
  const { nodes, edges } = useMemo(() => {
    const nodes: Node[] = [];
    const edges: Edge[] = [];

    data.forEach((res, i) => {
      const instrId = `instr-${i}`;
      nodes.push({ id: instrId, data: { label: res.instruction }, position: { x: 0, y: i * 100 } });

      res.statements.forEach((stmt, j) => {
        const stmtId = `stmt-${i}-${j}`;
        nodes.push({ id: stmtId, data: { label: stmt.statement }, position: { x: 250, y: i * 100 + j * 50 } });
        edges.push({ id: `e-${instrId}-${stmtId}`, source: instrId, target: stmtId });
      });
    });

    return { nodes, edges };
  }, [data]);

  return (
    <div style={{ width: '100%', height: '400px' }}>
      <ReactFlow nodes={nodes} edges={edges}>
        <Controls />
        <Background />
      </ReactFlow>
    </div>
  );
};

export default GraphView;
