use crate::features::equipment::dto::equipment_dto::EquipmentDto;
use crate::features::equipment::models::equipment_model::EquipmentModel;

pub fn dto_to_model(dto: &EquipmentDto) -> EquipmentModel {
    EquipmentModel {
        id: dto.id.clone(),
        public_id: dto.public_id.clone(),
        name: dto.name.clone(),
    }
}

pub fn model_to_dto(model: &EquipmentModel) -> EquipmentDto {
    EquipmentDto {
        id: model.id.clone(),
        public_id: model.public_id.clone(),
        name: model.name.clone(),
    }
}