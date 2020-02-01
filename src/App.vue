<template>
    <v-app>
        <v-container fluid class="pa-0" style="height: 100vh">
            <v-row no-gutters style="height: 100%">
                <v-col cols="3" class="overflow-auto">
                    <p class="title ps-4 pt-4">控制</p>
                    <div class="px-4">
                        <v-text-field label="点数" v-model.number="number"></v-text-field>
                    </div>

                    <v-toolbar dense flat class="pa-0">
                        <v-toolbar-title>图形</v-toolbar-title>
                        <v-spacer></v-spacer>
                        <v-btn icon color="primary" @click="addShape">
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                        <v-btn icon color="primary" @click="delShape">
                            <v-icon>mdi-minus</v-icon>
                        </v-btn>
                    </v-toolbar>

                    <v-list max-height="500px">
                        <v-list-item-group v-model.number="item" color="primary">
                            <v-list-item v-for="i in itemData.length"
                                :key="i"
                            >
                                <v-list-item-icon>
                                    <v-icon>mdi-shape</v-icon>
                                </v-list-item-icon>
                                <v-list-item-content>
                                    {{ "图形 " + i }}
                                </v-list-item-content>
                            </v-list-item>
                        </v-list-item-group>
                    </v-list>
                </v-col>
                <v-col cols="3" class="overflow-auto">
                    <p class="title ps-4 pt-4">编辑图形</p>
                    <p class="subtitle-1 ps-4" v-if="item >= 0 && item < itemData.length">
                        {{ "已选中：图形 " + (item + 1) }}
                    </p>
                    <p class="subtitle-1 ps-4" v-else>
                        未选中图形
                    </p>

                    <div class="ps-4 mb-4" v-if="item !== undefined && item >= 0 && item < itemData.length">
                        <v-row v-for="(i, index) in number" :key="i" no-gutters>
                            <v-col>
                                <input class="aaa" v-model.number="itemData[item][index][0]"
                                       @focusin="currentPoint = index" @focusout="currentPoint = -1"
                                />
                            </v-col>
                            <v-col>
                                <input class="aaa" v-model.number="itemData[item][index][1]"
                                       @focusin="currentPoint = index" @focusout="currentPoint = -1"
                                />
                            </v-col>
                        </v-row>
                    </div>

                    <div class="d-flex align-center justify-space-between">
                        <span class="ps-4 subtitle-1 align-center">当前图形：</span>
                            <svg width="200" height="200" viewBox="0 0 100 100" style="border: 1px solid #12345655"
                                 v-if="itemData[item]"
                            >
                                <path :d="d" fill="#12345655" stroke="none" stroke-width="1" stroke-linejoin="round"></path>
                                <circle v-for="(item, index) in itemData[item]"
                                        :key="index"
                                        :r="index === currentPoint ? '2' : '1.5'"
                                        :cx="(item[0] || 0) + 50"
                                        :cy="50 - (item[1] || 0)"
                                        :fill="index === currentPoint ? 'deeppink' : '#123456aa'"
                                ></circle>
                            </svg>
                    </div>


                </v-col>
                <v-col cols="6" class="ps-4">
                    <p class="title pt-4">结果</p>
                    <div class="d-flex align-center justify-space-between">
                        <svg width="200" height="200" viewBox="0 0 100 100" style="border: 1px solid #12345655"
                        >
                            <path v-for="(shape, index) in itemData"
                                  :key="'circle' + index"
                                  :d="computeD(index)"
                                  :fill="index === item ? '#123456cc' : '#12345622'"
                                  :stroke="index === item ? '#123456' : 'none'"
                            ></path>
                            <circle v-for="(item, index) in number * itemData.length"
                                    :key="index"
                                    :r="isCurrentPoint(index) ? '2' : '1.5'"
                                    :cx="itemData[Math.floor(index / number)][index % number][0] + 50"
                                    :cy="50 - itemData[Math.floor(index / number)][index % number][1]"
                                    :fill="isCurrentPoint(index) ? 'deeppink' : '#12345622'"
                            >
                            </circle>
                        </svg>

                        <v-btn color="primary" @click="calc">计算</v-btn>

                        <svg width="200" height="200" viewBox="0 0 100 100" style="border: 1px solid #12345655"
                        >
                            <path v-for="(shape, index) in itemData2"
                                  :key="'circle2-' + index"
                                  :d="computeD2(index)"
                                  :fill="index === item ? '#123456cc' : '#12345622'"
                                  :stroke="index === item ? '#123456' : 'none'"
                            ></path>
                            <circle v-for="(item, index) in number * itemData2.length"
                                    :key="index"
                                    :r="isCurrentPoint(index) ? '2' : '1.5'"
                                    :cx="itemData2[Math.floor(index / number)][index % number][0] * scale + 50"
                                    :cy="50 - itemData2[Math.floor(index / number)][index % number][1] * scale"
                                    :fill="isCurrentPoint(index) ? 'deeppink' : '#12345622'"
                            >
                            </circle>
                        </svg>
                    </div>
                    <div v-if="itemData2[item]" class="pt-4">
                        <v-row v-for="(i, index) in number" :key="i" no-gutters>
                            <v-col>
                                <input class="aaa" :value="itemData2[item][index][0]"
                                       @focusin="currentPoint = index" @focusout="currentPoint = -1"
                                />
                            </v-col>
                            <v-col>
                                <input class="aaa" :value="itemData2[item][index][1]"
                                       @focusin="currentPoint = index" @focusout="currentPoint = -1"
                                />
                            </v-col>
                        </v-row>
                    </div>
                </v-col>
            </v-row>
        </v-container>
    </v-app>
</template>

<script>
    // import * as wasm from "wasm";

    export default {
        name: 'App',

        components: {
        },

        data: () => ({
            number: 4,
            item: -1,
            itemData: [],
            itemData2: [],
            currentPoint: -1,
            scale: 40
        }),

        watch: {
            number: function (newValue) {
                let x = newValue || 0;
                for (let i = 0; i < this.itemData.length; i++) {
                    let temp = [];
                    for (let j = 0 ; j < x; j++) {
                        temp.push([0, 0]);
                    }
                    this.itemData[i] = temp;
                }
            }
        },

        methods: {
            isCurrentPoint(index) {
                let x = Math.floor(index / this.number) === this.item;
                let y = this.currentPoint === index % this.number;
                return x && y;
            },
            calc: function () {
                let x = new Float64Array(this.itemData.length * this.number * 2);
                for (let i = 0 ; i < this.itemData.length; i++) {
                    for (let j = 0; j < this.number; j++) {
                        x[i * this.number * 2 + j * 2] = this.itemData[i][j][0];
                        x[i * this.number * 2 + j * 2 + 1] = this.itemData[i][j][1];
                    }
                }

                let temp = this.wasm.calc(x, this.itemData.length, this.number);
                // deep copy
                this.itemData2 = JSON.parse(JSON.stringify(this.itemData));
                for (let i = 0 ; i < this.itemData.length; i++) {
                    for (let j = 0; j < this.number; j++) {
                        this.itemData2[i][j][0] = temp[i * this.number * 2 + j * 2];
                        this.itemData2[i][j][1] = temp[i * this.number * 2 + j * 2 + 1];
                    }
                }
            },
            computeD: function (index) {
                let data = this.itemData[index];
                if (!data) {
                    return "";
                }

                let temp = `M ${data[0][0] + 50} ${50 - data[0][1]}`;
                for (let i = 1; i < this.number; i++) {
                    temp += ` L ${data[i][0] + 50} ${50 - data[i][1]}`;
                }
                temp += " Z";
                return temp;
            },
            computeD2: function (index) {
                let data = this.itemData2[index];
                if (!data) {
                    return "";
                }

                let temp = `M ${data[0][0] * this.scale + 50} ${50 - data[0][1] * this.scale}`;
                for (let i = 1; i < this.number; i++) {
                    temp += ` L ${data[i][0] * this.scale + 50} ${50 - data[i][1] * this.scale}`;
                }
                temp += " Z";
                return temp;
            },
            addShape: function () {
                let temp = [];
                for (let i = 0; i < this.number; i++) {
                    temp.push([0, 0]);
                }
                this.itemData.push(temp);
            },
            delShape: function () {
                if (this.item >= 0 && this.item < this.itemData.length) {
                    this.itemData.splice(this.item, 1);
                }
            },
        },

        computed: {
            d: function () {
                let data = this.itemData[this.item];
                if (!data) {
                    return "";
                }

                let temp = `M ${data[0][0] + 50} ${50 - data[0][1]}`;
                for (let i = 1; i < this.number; i++) {
                    temp += ` L ${data[i][0] + 50} ${50 - data[i][1]}`;
                }
                temp += " Z";
                return temp;
            }
        }
    };
</script>

<style scoped>
    .aaa {
        width: 100%;
        /*background-color: #00000011;*/
        border: 1px solid #00000022;
        outline: none;
        padding: 0 8px;
        height: 32px;
    }
</style>
