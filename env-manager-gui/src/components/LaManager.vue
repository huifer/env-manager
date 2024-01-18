<script lang="ts" setup>
import {onMounted, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {dialog} from "@tauri-apps/api";
import {documentDir} from '@tauri-apps/api/path';

const columns = [

  {
    title: '语言',
    dataIndex: 'la',
    key: 'la',
  },
  {
    title: '路径',
    dataIndex: 'path',
    key: 'path',
  },
  {
    title: '别名',
    dataIndex: 'alias',
    key: 'alias',
  },
  {
    title: '是否使用',
    dataIndex: 'usingss',
    key: 'usingss',
  }, {
    title: '操作列',
    dataIndex: 'opera',
    key: 'opera',
  }
];

const open = ref<boolean>(false);

const showModal = () => {
  open.value = true;
};


const handleOk = async () => {
  console.log("当前数据", addEnvParam);
  await invoke("add_la", {
    param: addEnvParam
  }).then((d) => {
    initdata()

    addEnvParam.uid = "";
    addEnvParam.la = "";
    addEnvParam.path = "";
    addEnvParam.alias = "";
    addEnvParam.usingss = false
    console.log(d);
  })
  open.value = false;

};
const dataSource = reactive<any>({
  table: null
});
const addEnvParam = reactive({
  uid: "",
  la: "",
  path: "",
  alias: "",
  usingss: false,
})

async function initdata() {
  const d = await invoke("load_data");
  dataSource.table = d;
}

onMounted(async () => {
  await initdata();

})

const openFile = async () => {
  const result = await dialog.open({
    directory: true,
    multiple: false,
    defaultPath: await documentDir(),

  });
  if (Array.isArray(result)) {
    debugger
  } else if (result !== null) {
    console.log("选中的文件:", result)
    addEnvParam.path = result;
  } else {

  }
}

const usingPath = async (index: any, bol: boolean) => {
  console.log("索引 + ", index);
  await invoke("using_path", {index: index, bol: bol}).then(
      async () => {
        await initdata();
      }
  )
};
const deleteLa = async (index: any,uid: string) => {

  await invoke("delete_by_uid", {uid: uid, index: index}).then(
      async () => {
        await initdata();
      }
  )
};
</script>

<template>
  <div>
    <a-button type="primary" @click="showModal">增加环境</a-button>
    <a-table :columns="columns" :dataSource="dataSource.table">

      <template #bodyCell="{ column, record,index }">
        <template v-if="column.key === 'usingss'">
          <div>{{ record.usingss ? "使用" : "未使用" }}</div>
        </template>
        <template v-else-if="column.key ==='opera'">
          <a-button size="small" type="primary" @click="usingPath(index,!record.usingss)">
            {{ record.usingss ? "停用" : "启用" }}
          </a-button>
          <a-button danger size="small" type="primary" @click="deleteLa(index,record.uid)">
            删除
          </a-button>
        </template>
      </template>
    </a-table>
    <a-modal v-model:open="open" title="新增环境" @ok="handleOk">

      <a-form
          :label-col="{ span: 8 }"
          :model="addEnvParam"
          :wrapper-col="{ span: 16 }"
          autocomplete="off"
          name="basic"
      >
        <a-form-item
            :rules="[{ required: true, message: '请选择语言' }]"
            label="语言"
            name="la"
        >
          <a-select
              ref="select"
              v-model:value="addEnvParam.la"
              style="width: 120px"
          >
            <a-select-option value="Java">Java</a-select-option>
            <a-select-option value="Python">Python</a-select-option>
            <a-select-option value="Go">Go</a-select-option>
            <a-select-option value="Node">Node</a-select-option>
            <a-select-option value="Rust">Rust</a-select-option>
          </a-select>
        </a-form-item>
        <a-form-item
            :rules="[{ required: true, message: '请选择路径' }]"
            label="路径"
            name="path"
        >
          <a-space>

            <a-input v-model:value="addEnvParam.path" placeholder="请选择路径"/>
            <a-button @click="openFile">...</a-button>
          </a-space>
        </a-form-item>
        <a-form-item
            :rules="[{ required: true, message: '请输入别名' }]"
            label="别名"
            name="alias"
        >
          <a-input v-model:value="addEnvParam.alias" placeholder="请输入别名"/>
        </a-form-item>

      </a-form>

    </a-modal>
  </div>
</template>

<style scoped>

</style>