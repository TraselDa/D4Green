function toogle(data)
{
    var {y,commune}=data;
    const x = document.getElementById("data-details");
    if(x.style.display === "none")
    {
        x.style.flex ="3";
        x.style.display = "block";
        document.querySelector(".buttonVoir").innerHTML = "voir plus";
        document.getElementById(y.id).innerHTML = "voir moins";
        voir_details(commune);
    } else
    {
        x.style.flex ="0";
        x.style.display = "none";
        document.querySelector(".buttonVoir").innerHTML = "voir plus";
        document.getElementById(y.id).innerHTML = "voir plus";
    }

    window.onclick = function (event){
        if(event.target === document.getElementById("data-details") && x.style.display === "block") {
            document.getElementById("data-details").style.flex ="0";
            document.getElementById("data-details").style.display = "none";
            document.querySelector(".buttonVoir").innerHTML = "voir plus";
            document.getElementById(y.id).innerHTML = "voir plus";
        }
    }

}
var loadingDiv;
function voir_details(x)
{
    document.getElementById("_NOM_R_DEP").innerHTML = x["nomdep"] + " & "+ x["nomr"];
    document.getElementById("_NOM_R_DEP").style.fontWeight = "Bold";

    document.getElementById("_CLASSEMENT_").innerHTML = "N : " + x["classement"];
    document.getElementById("_CLASSEMENT_").style.fontWeight = "Bold" ;
    document.getElementById("_CLASSEMENT_").style.color = "Red" ;

    document.getElementById("_IRIS_._NC_").innerHTML = x["code_iris"] + x["nom_iris"];
    document.getElementById("_IRIS_._NC_").style.fontWeight = "Bold";

    document.getElementById("_POP_").innerHTML = "Population : " + x["population"];

    document.getElementById("_COM_").innerHTML = x["nom_com"];
    document.getElementById("_COM_").style.fontWeight = "Bold";
    document.getElementById("_COM_").style.fontSize = "25px";

    document.getElementById("_SCORE_G_").innerHTML = "SCORE Global : " + x["score_global"];
    document.getElementById("_SCORE_G_").style.fontWeight = "Bold";

    document.getElementById("_GLOBAL_ACCES_").innerHTML = "Global Acces : " + x["global_acces"];
    document.getElementById("_GLOBAL_ACCES_").style.fontWeight = "Bold";

    document.getElementById("_GLOBAL_COMP_").innerHTML = "Global Competences : " + x["global_competences"];
    document.getElementById("_GLOBAL_COMP_").style.fontWeight = "Bold";

    document.getElementById("_ACCES_A_LINFO_").innerHTML = "Acces a L information : " + x["acces_a_linformation"];
    document.getElementById("_ACCES_A_LINFO_").style.fontWeight = "Bold";
    document.getElementById("_ACCES_A_LINTERFACE_").innerHTML = "Acces aux interfaces numeriques : " + x["acces_aux_interfaces_numerique"];
    document.getElementById("_ACCES_A_LINTERFACE_").style.fontWeight = "Bold";

    document.getElementById("_COMPETENCES_ADMIN_").innerHTML = "Competences Administratives : " + x["competences_administratives"];
    document.getElementById("_COMPETENCES_ADMIN_").style.fontWeight = "Bold";
    document.getElementById("_COMPETENCES_NUM_").innerHTML = "Competences numeriques : " + x["competences_numeriques"];
    document.getElementById("_COMPETENCES_NUM_").style.fontWeight = "Bold";


}


window.addEventListener("DOMContentLoaded", (event) => {
    loadingDiv=document.getElementsByClassName("loading-overlay")[0];
    loadingDiv.style.display="none";
    loadDatatable();
});
function loadDatatable(page=1,id=undefined,region=undefined){
    if (window.XMLHttpRequest) {
        httpRequest = new XMLHttpRequest();
        httpRequest.onreadystatechange = function() {
            if (httpRequest.readyState === XMLHttpRequest.DONE) {
                if (httpRequest.status === 200) {
                    var arrCommunes = JSON.parse(this.responseText);
                    if(arrCommunes.length>0){
                        fillTableau(arrCommunes);
                    }
                } else {
                    alert('Il y a eu un problème avec la requête.');
                }
            }
        };
        var params={current_page:page};
        if(id!=undefined)
            params["last_id"]=parseInt(id);
        if(region!=undefined)
            params["region"]=region;
        httpRequest.open('POST', '/loadTable', true);
        httpRequest.setRequestHeader("Content-Type", "application/x-commune-paging;charset=UTF-8");
        httpRequest.send(JSON.stringify(params));
    }
}
function fillTableau(communes){
    var table = document.getElementById("table_body");
    table.innerHTML = "";
    for(var i=0;i<communes.length;i++){
        var commune=communes[i];
        var communeId=commune['id'];
        var row = table.insertRow();
        row.dataset.key=communeId;
        row.insertCell().innerText=commune['nom_com'];
        row.insertCell().innerText=commune['nomdep'];
        row.insertCell().innerText=commune['nomr'];
        row.insertCell().innerText=commune['population'];
        row.insertCell().innerText=commune['score_global'];
        var button=document.createElement("button");
        button.className="buttonVoir";
        button.innerText="Voir Plus";
        button.id="voir_"+communeId;
        button.addEventListener ("click",toogle.bind(this,{y:button,commune:commune}));
        row.insertCell().appendChild(button);

    }
}
function navigateNext(){
var tables=document.getElementById("table-items");
var body=document.getElementById("table_body");
var current_page=parseInt(tables.dataset.index);

current_page+=1;
tables.dataset.index=current_page;
var t=body.rows.length;
var last_row =body.rows[t-1];
loadDatatable(current_page,last_row.dataset.key);
}
function navigatePrev(){
    var current_page=parseInt(tables.dataset.index);
    current_page-=1;
    var t=body.rows.length;
    var first_row =body.rows[0];
    tables.dataset.index=current_page;
    loadDatatable(current_page,last_row.dataset.key);
}
function selectRegion(parent){
    var value = document.getElementById("box-region").value;
    loadDatatable(1,undefined,value)
}