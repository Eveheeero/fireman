import React, { useState } from "react";
import logoBackgrounded from "./assets/logo colored 512.png";
import "./App.css";
import Navigation from "./Navigation";
import LogBar from "./LogBar";
import { closestCenter, DndContext, DragEndEvent, DragOverlay, DragStartEvent, KeyboardSensor, PointerSensor, UniqueIdentifier, useSensor, useSensors } from "@dnd-kit/core";
import { arrayMove, horizontalListSortingStrategy, SortableContext, sortableKeyboardCoordinates, useSortable } from "@dnd-kit/sortable";
import { PanelGroup, Panel, PanelResizeHandle } from "react-resizable-panels";
import { CSS } from '@dnd-kit/utilities';
import AssemblyPanel from "./AssemblyPanel";
import IRPanel from "./IRPanel";
import ASTPanel from "./ASTPanel";
import SectionPanel from "./SectionPanel";


interface Panel {
  id: string;
  content: React.ReactNode;
}

function SortablePanel({ id, content }: Readonly<Panel>) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    zIndex: isDragging ? 10 : 1,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      className="h-full w-full flex flex-col"
    >
      <div
        {...attributes}
        {...listeners}
        className="h-4 w-full bg-gray-500 cursor-grab rounded-t-lg flex-shrink-0 z-10"
      ></div>
      <div className="flex-1 bg-gray-900/20 rounded-b-lg overflow-hidden">
        {content}
      </div>
    </div>
  );
}

function App() {
  const [panels, setPanels] = useState<Panel[]>([
    { id: 'section-panel', content: <SectionPanel /> },
    { id: 'asm-panel', content: <AssemblyPanel /> },
    { id: 'ir-panel', content: <IRPanel /> },
    { id: 'ast-panel', content: <ASTPanel /> },
  ]);
  const [draggingPanelId, setDraggingPanelId] = useState<UniqueIdentifier | null>(null);

  // dragging state
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragStart = (event: DragStartEvent) => {
    setDraggingPanelId(event.active.id);
  };
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (!over) {
      setDraggingPanelId(null);
      return;
    }

    if (active.id !== over.id) {
      setPanels((items) => {
        const oldIndex = items.findIndex((item) => item.id === active.id);
        const newIndex = items.findIndex((item) => item.id === over.id);
        return arrayMove(items, oldIndex, newIndex);
      });
    }
    setDraggingPanelId(null);
  };
  const handleDragCancel = () => {
    setDraggingPanelId(null);
  };

  const draggingPanel = draggingPanelId ? panels.find((panel) => panel.id === draggingPanelId) : null;

  return (
    <main className="h-screen overflow-hidden flex flex-col">
      <Navigation />{/* <div className="row">
        <img src="/logo transparent.svg" className="logo" alt="transparent logo" />
        <img src={logoBackgrounded} className="logo" alt="backgrounded logo" />
      </div> */}
      <div className="flex-1 flex items-center justify-center p-4 text-black overflow-hidden">
        <DndContext sensors={sensors}
          collisionDetection={closestCenter}
          onDragStart={handleDragStart}
          onDragEnd={handleDragEnd}
          onDragCancel={handleDragCancel}>
          <SortableContext
            items={panels.map((p) => p.id)}
            strategy={horizontalListSortingStrategy}
          >
            <PanelGroup direction="horizontal" className="w-full h-full">
              {panels.map((content, index) => (
                <React.Fragment key={content.id}>
                  <Panel defaultSize={100 / panels.length} className="h-full">
                    <SortablePanel
                      id={content.id}
                      content={content.content}
                    />
                  </Panel>
                  {index < panels.length - 1 && (
                    <PanelResizeHandle className="w-1 bg-gray-300 hover:bg-blue-500 transition-colors duration-200 cursor-ew-resize flex items-center justify-center">
                      <div className="w-1 h-10 bg-gray-500 rounded-full"></div>
                    </PanelResizeHandle>
                  )}
                </React.Fragment>
              ))}
            </PanelGroup>
          </SortableContext>

          <DragOverlay>
            {draggingPanel ? (
              <SortablePanel
                id={draggingPanel.id}
                content={draggingPanel.content}
              />
            ) : null}
          </DragOverlay>
        </DndContext>
      </div>

      <LogBar />
    </main >
  );
}

export default App;
