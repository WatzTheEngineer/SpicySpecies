const { resolveResource } = window.__TAURI__.path;
const { readTextFile } = window.__TAURI__.fs;
const { invoke } = window.__TAURI__.tauri;

const LOCALE = String(await invoke("getlocale", {})).slice(0, 2);

try{
    var langpath = await resolveResource('lang/' + LOCALE + '.json');
} catch (error){
    var langpath = await resolveResource('lang/en.json');
}

const LANG = JSON.parse(await readTextFile(langpath));

const SPECIES_BOX = LANG.specie_box;
const SQUARE = LANG.square;

const ALERT_ON_STATS = LANG.alert_on_stats;

const SHANNON_H2 = LANG.shannon_h2;
const SHANNON_SPAN = LANG.shannon_span;
const PIELOU_H2 = LANG.pielou_h2;
const PIELOU_SPAN = LANG.pielou_span;
const SIMPSON_H2 = LANG.simpson_h2;
const SIMPSON_SPAN = LANG.simpson_span;
const DICE_H2 = LANG.dice_h2;
const AND = LANG.and;
const DICE_SPAN = LANG.dice_span;
const JACCARD_H2 = LANG.jaccard_h2;
const JACCARD_SPAN = LANG.jaccard_span;
const PERCENTAGE = "%";

const NEWSPECIE = LANG.new_specie;
const ADDSQUARE = LANG.add_square;
const RMSQUARE = LANG.rm_square;
const GETSTATS = LANG.get_stats;
const BACKTO = LANG.back_to;

var mainDiv = document.getElementById("main");
var statsDiv = document.getElementById("stats");

function AddNewSpecie(square_index) {
    mainDiv.getElementsByClassName("square")[Number(square_index)].getElementsByClassName("main-show")[0].innerHTML += SPECIES_BOX;
}

function initApp() {
    
    document.getElementsByClassName("new-specie")[0].innerText = NEWSPECIE;
    document.getElementById("add-square").innerText = ADDSQUARE;
    document.getElementById("rm-square").innerText = RMSQUARE;
    document.getElementById("get-stats").innerText = GETSTATS;
    document.getElementById("back-to").innerText = BACKTO;
    
    document.getElementById("get-stats").addEventListener("click", () => {
        let canCook = true;
        Array.from(mainDiv.getElementsByClassName("square")).forEach((e) => {
            if (e.children[0].children.length == 0 && canCook) {
                alert(ALERT_ON_STATS);
                canCook = false;
            }
        })
        if (canCook) cookAllStats();
    })

    document.getElementById("back-to").addEventListener("click", () => {
        backToSquares();
    })

    document.getElementsByClassName("new-specie")[0].addEventListener("click", () => {
        AddNewSpecie(0);
    })

    document.getElementById("add-square").addEventListener("click", function () {
        let square_number = mainDiv.getElementsByClassName("square").length.valueOf();
        let new_square = document.createElement('div');
        new_square.className = 'square';
        new_square.innerHTML = SQUARE;
        new_square.id = square_number;
        mainDiv.insertBefore(new_square, mainDiv.childNodes[mainDiv.childNodes.length - 2]);
        mainDiv.childNodes[mainDiv.childNodes.length - 3].childNodes[1].childNodes[0].addEventListener("click", () => {
            AddNewSpecie(square_number);
        })
    });

    document.getElementById("rm-square").addEventListener("click", function () {
        let squareCount = mainDiv.getElementsByClassName("square").length;
        if (squareCount > 1) {
            mainDiv.getElementsByClassName("square")[squareCount - 1].remove();
        }
    });

    console.log("%c APP INITIALIZED !", "color: #bada55");
}

async function cookAllStats() {

    mainDiv.style.display = "none";
    statsDiv.style.display = "flex";

    statsDiv.children[0].innerHTML = ""; // Reset stats page

    let squareList = Array.from(mainDiv.getElementsByClassName("square"));
    let statsMatrix = [], speciesNames = [];
    squareList.forEach(sqr => {
        Array.from(sqr.getElementsByClassName("specie-box")).forEach(box => {
            let name = box.querySelector(".specie-name").innerText;
            if (!speciesNames.includes(name)) {
                speciesNames.push(name);
            }
        });
    });
    squareList.forEach(() => {
        let row = [];
        for (let i = 0; i < speciesNames.length; i++) {
            row.push(0);
        }
        statsMatrix.push(row);
    });
    squareList.forEach((sqr, index) => {
        let speciesValues = {};
        Array.from(sqr.getElementsByClassName("specie-box")).forEach(box => {
            let name = box.querySelector(".specie-name").innerText;
            let value = Number(box.children[1].children[1].innerText);
            speciesValues[name] = value;
        });

        speciesNames.forEach((name, i) => {
            if (speciesValues.hasOwnProperty(name)) {
                statsMatrix[index][i] = speciesValues[name];
            }
        });
    });

    let cooked = await invoke("cook", { statsVector: statsMatrix });
    buildStatPage(cooked);

}

function buildStatPage(cooked) {

    let shannon = document.createElement("div");
    shannon.id = "shannon";
    shannon.className = "statpoint";
    shannon.appendChild(document.createElement("h2"));
    shannon.appendChild(document.createElement("span"));
    shannon.children[0].innerText = SHANNON_H2 + cooked[0][0];
    shannon.children[1].innerHTML = SHANNON_SPAN;

    statsDiv.children[0].appendChild(shannon);

    let pielou = document.createElement("div");
    pielou.innerHTML = shannon.innerHTML;
    pielou.id = "pielou";
    pielou.className = "statpoint";
    pielou.children[0].innerText = PIELOU_H2 + cooked[0][1] + PERCENTAGE;
    pielou.children[1].innerHTML = PIELOU_SPAN;

    statsDiv.children[0].appendChild(pielou);

    let simpson = document.createElement("div");
    simpson.innerHTML = shannon.innerHTML;
    simpson.id = "simpson";
    simpson.className = "statpoint";
    simpson.children[0].innerText = SIMPSON_H2 + cooked[1][0] + PERCENTAGE;
    simpson.children[1].innerHTML = SIMPSON_SPAN;

    statsDiv.children[0].appendChild(simpson);

    let firstVectorIndex = 1, secondVectorIndex = 2;
    let dice = document.createElement("div");
    dice.className = "statpoint";
    dice.id = "dice";

    cooked[2].forEach(e => {

        dice.appendChild(document.createElement("h2"));
        dice.appendChild(document.createElement("span"));
        dice.children[dice.children.length - 2].innerText = DICE_H2 + firstVectorIndex + AND + secondVectorIndex + " : " + e
        dice.children[dice.children.length - 1].innerHTML = DICE_SPAN;

        if (secondVectorIndex == Array.from(mainDiv.getElementsByClassName("square")).length) {
            firstVectorIndex++
            secondVectorIndex = firstVectorIndex + 1
        } else {
            secondVectorIndex++
        }

    });

    if (dice.children.length > 0) { statsDiv.children[0].appendChild(dice) };

    firstVectorIndex = 0;
    secondVectorIndex = 0;
    let jaccard = document.createElement("div");
    jaccard.className = "statpoint"
    jaccard.id = "jaccard"

    cooked[3].forEach(e => {

        jaccard.appendChild(document.createElement("h2"));
        jaccard.appendChild(document.createElement("span"));
        jaccard.children[jaccard.children.length - 2].innerText = JACCARD_H2 + firstVectorIndex + AND + secondVectorIndex + " : " + e + PERCENTAGE
        jaccard.children[jaccard.children.length - 1].innerHTML = JACCARD_SPAN;

        if (secondVectorIndex == Array.from(mainDiv.getElementsByClassName("square")).length) {
            firstVectorIndex++;
            secondVectorIndex = firstVectorIndex + 1;
        } else {
            secondVectorIndex++;
        }

    });

    if (jaccard.children.length > 0) { statsDiv.children[0].appendChild(jaccard); };
}

function backToSquares() {
    statsDiv.style.display = "none";
    mainDiv.style.display = "flex";
}

function main() {
    initApp();
}

main();