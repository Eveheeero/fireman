import React, { useMemo, useState } from "react";
import "./App.css";
import Navigation from "./Navigation";
import LogBar from "./LogBar";
import {
  closestCenter,
  DndContext,
  DragEndEvent,
  DragOverlay,
  DragStartEvent,
  KeyboardSensor,
  PointerSensor,
  UniqueIdentifier,
  useDroppable,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  horizontalListSortingStrategy,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
} from "@dnd-kit/sortable";
import { PanelGroup, Panel, PanelResizeHandle } from "react-resizable-panels";
import { CSS } from "@dnd-kit/utilities";
import AssemblyPanel from "./AssemblyPanel";
import IRPanel from "./IRPanel";
import ASTPanel from "./ASTPanel";
import SectionPanel from "./SectionPanel";

// ---- Types ----
interface PanelItem {
  id: string;
  content: React.ReactNode;
}
type ContainerId = "top" | "bottom";

// ---- Sortable Panel (개별 패널 카드) ----
function SortablePanel({ id, content }: Readonly<PanelItem>) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } =
    useSortable({ id });

  const style: React.CSSProperties = {
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
      />
      <div className="flex-1 bg-gray-900/20 rounded-b-lg overflow-hidden">
        {content}
      </div>
    </div>
  );
}


function DroppableRow({
  id,
  children,
}: {
  id: ContainerId;
  children: React.ReactNode;
}) {
  const { setNodeRef, isOver } = useDroppable({ id });
  return (
    <div
      ref={setNodeRef}
      className={`h-full w-full ${isOver ? "outline outline-2 outline-blue-400" : ""}`}
    >
      {children}
    </div>
  );
}


function SortableRow({
  id,
  items,
}: {
  id: ContainerId;
  items: PanelItem[];
}) {
  return (
    <DroppableRow id={id}>
      <SortableContext
        // 같은 줄에서는 가로 정렬 전략
        items={items.map((i) => i.id)}
        strategy={horizontalListSortingStrategy}
      >
        <PanelGroup direction="horizontal" className="w-full h-full">
          {items.length === 0 ? (null) : (
            items.map((item, index) => (
              <React.Fragment key={item.id}>
                <Panel defaultSize={100 / items.length} className="h-full">
                  <SortablePanel id={item.id} content={item.content} />
                </Panel>
                {index < items.length - 1 && (
                  <PanelResizeHandle className="w-1 bg-gray-300 hover:bg-blue-500 transition-colors duration-200 cursor-ew-resize flex items-center justify-center">
                    <div className="w-1 h-10 bg-gray-500 rounded-full" />
                  </PanelResizeHandle>
                )}
              </React.Fragment>
            ))
          )}
        </PanelGroup>
      </SortableContext>
    </DroppableRow>
  );
}

function App() {
  // 초기 4패널 정의
  const allPanels = useMemo<PanelItem[]>(
    () => [
      { id: "section-panel", content: <SectionPanel /> },
      { id: "asm-panel", content: <AssemblyPanel /> },
      { id: "ir-panel", content: <IRPanel /> },
      { id: "ast-panel", content: <ASTPanel /> },
    ],
    []
  );

  // 위/아래 두 줄 레이아웃 상태
  const [layout, setLayout] = useState<Record<ContainerId, PanelItem[]>>({
    top: [allPanels[0], allPanels[1]],    // 위 줄: 2개
    bottom: [allPanels[2], allPanels[3]], // 아래 줄: 2개
  });

  const [draggingPanelId, setDraggingPanelId] = useState<UniqueIdentifier | null>(null);

  // dnd sensors
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  );

  // 유틸: 아이템이 속한 컨테이너 찾기
  const findContainer = (id: UniqueIdentifier): ContainerId | null => {
    if (layout.top.some((p) => p.id === id)) return "top";
    if (layout.bottom.some((p) => p.id === id)) return "bottom";
    return null;
  };

  // 유틸: 아이템 객체 찾기
  const getItem = (id: UniqueIdentifier) =>
    layout.top.find((p) => p.id === id) ?? layout.bottom.find((p) => p.id === id) ?? null;

  const handleDragStart = (event: DragStartEvent) => {
    setDraggingPanelId(event.active.id);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (!over) {
      setDraggingPanelId(null);
      return;
    }

    const activeId = active.id;
    const overId = over.id;

    const activeContainer = findContainer(activeId);
    const overContainer =
      (overId === "top" || overId === "bottom") ? (overId as ContainerId) : findContainer(overId);

    if (!activeContainer || !overContainer) {
      setDraggingPanelId(null);
      return;
    }

    // 같은 줄에서 위치만 변경
    if (activeContainer === overContainer) {
      if (activeId !== overId) {
        setLayout((prev) => {
          const items = prev[activeContainer];
          const oldIndex = items.findIndex((i) => i.id === activeId);
          const newIndex = items.findIndex((i) => i.id === overId);
          const next = arrayMove(items, oldIndex, newIndex);
          return { ...prev, [activeContainer]: next };
        });
      }
    } else {
      // 다른 줄로 이동 (상/하 이동)
      setLayout((prev) => {
        const fromItems = [...prev[activeContainer]];
        const toItems = [...prev[overContainer]];

        const moving = fromItems.find((i) => i.id === activeId);
        if (!moving) return prev;

        // from에서 제거
        const fromIndex = fromItems.findIndex((i) => i.id === activeId);
        fromItems.splice(fromIndex, 1);

        // to에 삽입 위치 계산
        const overIndex =
          overId === overContainer
            ? toItems.length // 빈 영역이나 줄 자체에 떨어뜨렸을 때는 맨 뒤
            : Math.max(0, toItems.findIndex((i) => i.id === overId));

        toItems.splice(overIndex, 0, moving);

        return {
          ...prev,
          [activeContainer]: fromItems,
          [overContainer]: toItems,
        };
      });
    }

    setDraggingPanelId(null);
  };

  const handleDragCancel = () => setDraggingPanelId(null);

  const draggingPanel = draggingPanelId ? getItem(draggingPanelId) : null;

  return (
    <main className="h-screen overflow-hidden flex flex-col">
      <div className="relative z-50">
        <Navigation />
      </div>

      <div className="flex-1 flex items-center justify-center p-4 text-black overflow-hidden">
        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          onDragStart={handleDragStart}
          onDragEnd={handleDragEnd}
          onDragCancel={handleDragCancel}
        >
          {/* 세로로 두 줄(Stack) + 줄 사이 리사이즈 */}
          <PanelGroup direction="vertical" className="w-full h-full">
            {/* ---- TOP ROW ---- */}
            <Panel defaultSize={50} className="h-screen">
              <SortableRow id="top" items={layout.top} />
            </Panel>
            <PanelResizeHandle className="h-1 bg-gray-300 hover:bg-blue-500 transition-colors duration-200 cursor-ns-resize flex items-center justify-center">
              <div className="h-1 w-10 bg-gray-500 rounded-full" />
            </PanelResizeHandle>
            {/* ---- BOTTOM ROW ---- */}
            <Panel defaultSize={50} className="h-screen">
              <SortableRow id="bottom" items={layout.bottom} />
            </Panel>
          </PanelGroup>

          <DragOverlay>
            {draggingPanel ? (
              <SortablePanel id={draggingPanel.id} content={draggingPanel.content} />
            ) : null}
          </DragOverlay>
        </DndContext>
      </div>

      <div className="relative z-50">
        <LogBar />
      </div>
    </main>
  );
}

export default App;
