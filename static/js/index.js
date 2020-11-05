function toogle(y)
{

    const x = document.getElementById("data-details");
    if(x.style.display === "none")
    {
        document.getElementById("data-details").style.flex ="3";
        document.getElementById("data-details").style.display = "block";
        document.querySelector(".buttonVoir").innerHTML = "voir plus";
        document.getElementById(y.id).innerHTML = "voir moins";
        voir_details(y);
    } else
    {
        document.getElementById("data-details").style.flex ="0";
        document.getElementById("data-details").style.display = "none";
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
    loadDatatable();
});
function loadDatatable(page=1){
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
        var params={page:page};
        httpRequest.open('POST', '/loadTable', true);
        httpRequest.send(JSON.stringify(params));
    }
}
function fillTableau(communes){
    var table = document.getElementById("table_body");
    table.innerHTML = "";
    for(var i=0;i<communes.length;i++){
        var commune=communes[i];
        var row = table.insertRow();
        var cell1 = row.insertCell();
        cell1.innerText=commune['nomdep'];
        var cell2 = row.insertCell();
        cell2.innerText=commune['nomr'];
    }
}
function navigateNext(){
var tables=document.getElementById("table-items");
var current_page=parseInt(tables.dataset.index);
current_page+=1;
tables.dataset.index=current_page;
loadDatatable(current_page);
}
function navigatePrev(){
    var current_page=parseInt(tables.dataset.index);
    current_page-=1;
    tables.dataset.index=current_page;
    loadDatatable(current_page);
}