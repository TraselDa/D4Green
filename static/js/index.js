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
    document.getElementById("titre").innerHTML = "details" + x.id;
}


window.addEventListener("DOMContentLoaded", (event) => {
    loadingDiv=document.getElementsByClassName("loading-overlay")[0];
    loadingDiv.style.display="none";
    loadDatatable();
});
function loadDatatable(page=1,id=undefined,region=undefined,departement=undefined,commune){
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
        if(departement!=undefined)
            params["departement"]=departement;
        if(commune!=undefined)
            params["commune"]=commune;
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
function  selectDepartement(parent){
    var departement = document.getElementById("box-departement").value;
    var region = document.getElementById("box-region").value;
    region=region=="Tout"?undefined:region;
    loadDatatable(1,undefined,region,departement)
}
function selectCommune(parent){
    var region = document.getElementById("box-region").value;
    region=region=="Tout"?undefined:region;
    var departement = document.getElementById("box-departement").value;
    departement=departement=="Tout"?undefined:departement;
    var commune = document.getElementById("box-commune").value;

    loadDatatable(1,undefined,region,departement,commune)
}