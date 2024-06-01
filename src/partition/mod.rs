use serde::Serialize;
use serde_json::Value;

pub mod get_partition_from_json;

#[derive(Serialize, Debug)]
pub struct Partition
{
    pub is_unallocated: bool,
    pub number: Option<u64>,
    pub start_sector: String,
    pub end_sector: String,
    pub size_sector: String,
    pub partition_type: String,
    pub partition_type_uuid: Option<String>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub filesystem: Option<String>,
    pub flags: Option<Vec<String>>
}

impl Partition
{
    pub fn new(is_unallocated: bool, number: Option<u64>,
        start_sector: String,
        end_sector: String,
        size_sector: String,
        partition_type: String,
        partition_type_uuid: Option<String>,
        uuid: Option<String>,
        name: Option<String>,
        filesystem: Option<String>,
        flags: Option<Vec<String>>) -> Self
    {
        Self { number, start_sector, end_sector, size_sector, partition_type, partition_type_uuid, uuid, name, filesystem, flags, is_unallocated }
    }

    pub fn from_value_vec(partitions: Vec<Value>) -> Vec<Self>
    {
        let mut vec: Vec<Self> = Vec::new();

        for i in partitions
        {
            let size_sector = i["size"].as_str().expect("size is null").to_string();

            {
                let size_sector = size_sector.chars().filter(|char| char.is_numeric()).collect::<String>();

                if size_sector.parse::<u128>().unwrap() < 2048
                {
                    continue;
                }
            }

            let start_sector = i["start"].as_str().expect("start is null").to_string();
            let end_sector = i["end"].as_str().expect("end is null").to_string();
            let partition_type = i["type"].as_str().expect("type is null").to_string();

            let number = {
                let exist = !i["number"].is_null();
                
                if exist { Some(i["number"].as_u64().unwrap()) } else { None }
            };

            let partition_type_uuid = {
                let exist = !i["type-uuid"].is_null();
                
                if exist { Some(i["type-uuid"].as_str().unwrap().to_string()) } else { None }
            };

            let uuid = {
                let exist = !i["uuid"].is_null();
                
                if exist { Some(i["uuid"].as_str().unwrap().to_string()) } else { None }
            };

            let name = {
                let exist = !i["name"].is_null();
                
                if exist { Some(i["name"].as_str().unwrap().to_string()) } else { None }
            };

            let filesystem = {
                let exist = !i["filesystem"].is_null();
                
                if exist { Some(i["filesystem"].as_str().unwrap().to_string()) } else { None }
            };

            let flags = {
                let exist = !i["flags"].is_null();
                
                if exist
                {
                    Some(i["flags"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|flag| flag.as_str().unwrap().to_string())
                        .collect::<Vec<String>>())
                }
                else { None }
            };

            if i["type"].as_str().unwrap() == "free"
            {
                vec.push(Self::new(
                    true,
                    number,
                    start_sector,
                    end_sector,
                    size_sector,
                    partition_type,
                    partition_type_uuid,
                    uuid,
                    name,
                    filesystem,
                    flags
                ));
            }
            else
            {
                vec.push(Self::new(
                    false,
                    number,
                    start_sector,
                    end_sector,
                    size_sector,
                    partition_type,
                    partition_type_uuid,
                    uuid,
                    name,
                    filesystem,
                    flags
                ));
            }
        }

        vec
    }
}
