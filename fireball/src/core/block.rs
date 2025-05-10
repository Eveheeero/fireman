//! 프로그램을 분석하여 각 섹션별로 나온 "Block"을 정의하는 모듈

use super::{Address, Relation, Section};
use crate::ir::IrBlock;
use std::sync::{Arc, RwLock, RwLockReadGuard};

/// 분석에 의해 생성된 어셈블리단위 블럭
///
/// 해당 구조체는 한 지점에서 jmp XXXX라는 명령어가 수행되어졌을 때, XXXX을 시작 주소로 하나의 블럭이 생성되며
/// 해당 블럭별로 가상 코드를 작성하여 디컴파일을 진행한다.
#[derive(Debug)]
pub struct Block {
    /// 블럭의 아이디
    id: usize,
    /// 블럭의 이름
    name: Option<String>,
    /// 블럭의 시작 주소
    start_address: Address,
    /// 블럭의 사이즈
    block_size: Option<u64>,
    /// 현재 블럭과 연관되어있는 블럭
    connected_from: RwLock<Vec<Relation>>,
    /// 현재 블럭과 연관된 블럭
    connected_to: RwLock<Vec<Relation>>,
    /// 블럭의 섹션
    section: Option<Arc<Section>>,

    /* 분석에 의해 생성된 데이터 */
    /// 블럭의 IR 데이터
    ir: RwLock<Option<IrBlock>>,
}

impl Block {
    /// 블럭을 생성한다.
    /// 해당 구조체는 직접적으로 생성될 수 없으며, blocks구조체에 의해 생성되고 관리된다.
    ///
    /// ### Arguments
    /// - `id: usize` - 블럭의 아이디
    /// - `name: Option<String>` - 블럭의 이름
    /// - `start_address: Address` - 블럭의 시작주소
    /// - `block_size: Option<u64>,` - 블럭의 사이즈
    ///
    /// ### Returns
    /// - `Arc<Self>` - 생성된 블럭
    ///
    /// ### Todo
    /// - name은 실행 파일의 구조체 정보에 담겨있는 이름에 따라 지정될 예정이다.
    /// - name은 사용자가 지정할 수 있도록 할 예정이다.
    pub(super) fn new(
        id: usize,
        name: Option<String>,
        start_address: Address,
        block_size: Option<u64>,
    ) -> Arc<Self> {
        let section = start_address.get_section();
        Arc::new(Self {
            id,
            name,
            start_address,
            block_size,
            connected_from: Default::default(),
            connected_to: Default::default(),
            section,
            ir: Default::default(),
        })
    }

    /// 블럭의 아이디를 반환한다.
    ///
    /// ### Returns
    /// - `usize` - 블럭의 아이디
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// 블럭의 이름을 반환한다.
    ///
    /// ### Returns
    /// - `Option<&String>` - 블럭의 이름
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// 블럭의 시작 주소를 반환한다.
    ///
    /// ### Returns
    /// - `&Address` - 블럭의 시작 주소
    pub fn get_start_address(&self) -> &Address {
        &self.start_address
    }

    /// 블럭의 사이즈를 반환한다.
    ///
    /// ### Returns
    /// - `Option<&u64>` - 블럭의 사이즈
    pub fn get_block_size(&self) -> Option<&u64> {
        self.block_size.as_ref()
    }

    /// 어떤 블럭이 해당 블럭에서 연결되어있는지를 반환한다.
    ///
    /// ### Returns
    /// - `RwLockReadGuard<Vec<Relation>>` - 연결된 블럭들
    pub fn get_connected_from(&self) -> RwLockReadGuard<Vec<Relation>> {
        self.connected_from.read().unwrap()
    }

    /// 해당 블럭이 어떤 블럭으로 연결되어있는지를 반환한다.
    ///
    /// ### Returns
    /// - `RwLockReadGuard<Vec<Relation>>` - 연결된 블럭들
    pub fn get_connected_to(&self) -> RwLockReadGuard<Vec<Relation>> {
        self.connected_to.read().unwrap()
    }

    /// 블럭이 어떤 섹션에 해당하는지를 반환한다.
    ///
    /// ### Returns
    /// - `&Option<Arc<Section>>` - 블럭이 속한 섹션
    pub fn get_section(&self) -> Option<&Arc<Section>> {
        self.section.as_ref()
    }

    /// 어떤 블럭이 해당 블럭에 연결되어 있는지를 추가한다.
    ///
    /// ### Arguments
    /// - `relation: Relation` - 해당 블럭으로 향하는 블럭
    pub(crate) fn add_connected_from(&self, relation: Relation) {
        self.connected_from.write().unwrap().push(relation);
    }

    /// 해당 블럭이 어떤 블럭에 연결되어 있는지를 추가한다.
    ///
    /// ### Arguments
    /// - `relation: Relation` - 해당 블럭이 향하는 블럭
    pub(crate) fn add_connected_to(&self, relation: Relation) {
        self.connected_to.write().unwrap().push(relation);
        debug_assert!(
            self.connected_to.read().unwrap().len() <= 2,
            "한 블럭에는 최대 두 개의 블럭이 연결될 수 있습니다."
        );
    }
    /// 블럭의 IR 데이터를 반환한다.
    ///
    /// ### Returns
    /// - `RwLockReadGuard<Option<IrBlock>>` - 블럭의 IR 데이터
    pub fn get_ir(&self) -> RwLockReadGuard<Option<IrBlock>> {
        self.ir.read().unwrap()
    }
    /// 블럭의 IR 데이터를 설정한다.
    ///
    /// ### Arguments
    /// - `ir: IrBlock` - 블럭의 IR 데이터
    pub fn set_ir(&self, ir: IrBlock) {
        *self.ir.write().unwrap() = Some(ir);
    }
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
