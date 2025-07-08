$(document).ready(function () {

    $('#btn_query').click(function () {
        $(this).prop('disabled', true);
        query();
        setTimeout(function() {
            $('#btn_query').prop('disabled', false);
        }, 2000);
    });


    $('#btn_query_sec').click(function () {
        $(this).prop('disabled', true);
        query_sec();
        setTimeout(function() {
            $('#btn_query_sec').prop('disabled', false);
        },2000)
    });


    $(document).ready(function () {
        $('.dns-checkbox_sec').on('change', function () {
            $('.dns-checkbox_sec').not(this).prop('checked', false);
        });
    });

    function query_sec() {
        var domainSec = $('#inputDomainSec').val();
        //记录类型
        var recordTypeSec = $('input[name="record_type_sec"]:checked').val();
        //dns服务商列表
        var dnsServerSecArr = [];
        $('.dns-checkbox_sec:checked').each(function () {
            dnsServerSecArr.push($(this).val());
        });

        var is_valid = true;
        if (isEmpty(domainSec)) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "域名不能为空!",
            });
        }


        if (isEmpty(recordTypeSec)) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "记录类型不能为空!",
            });
        }
        if (dnsServerSecArr.length === 0) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "请选择一个dns服务商!",
            });
        }
        if (!isEmpty(domainSec)) {
            if (!domainRegex.test(domainSec)) {
                is_valid = false;
                Swal.fire({
                    icon: "error",
                    text: "无效的域名格式!",
                });
            }
        }
        if (is_valid) {
            v1_query_sec(domainSec, recordTypeSec, dnsServerSecArr);
        }

    }


    function query() {
        //域名
        var domain = $('#inputDomain').val();
        //自定义dns服务器
        var cusDns = $('#inputCusDns').val();
        //记录类型
        var recordType = $('input[name="record_type"]:checked').val();
        //dns服务商列表
        var dnsServerArr = [];
        $('.dns-checkbox:checked').each(function () {
            dnsServerArr.push($(this).val());
        });

        var is_valid = true;
        if (isEmpty(domain)) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "域名不能为空!",
            });
        }

        if (!isEmpty(cusDns)) {
            if (!ipv4Regex.test(cusDns)) {
                is_valid = false;
                Swal.fire({
                    icon: "error",
                    text: "无效的dns服务器地址!",
                });
            }
        }

        if (isEmpty(recordType)) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "记录类型不能为空!",
            });
        }
        if (dnsServerArr.length === 0 && isEmpty(cusDns)) {
            is_valid = false;
            Swal.fire({
                icon: "error",
                text: "请选择一个dns服务商或者输入自定义dns服务器!",
            });
        }
        if (!isEmpty(domain)) {
            if (!domainRegex.test(domain)) {
                is_valid = false;
                Swal.fire({
                    icon: "error",
                    text: "无效的域名格式!",
                });
            }
        }
        if (is_valid) {
            v1_query(domain, cusDns, recordType, dnsServerArr);
        }
    }


    function v1_query_sec(domainSec, recordTypeSec, dnsServerSecArr) {
        let fullDnsServerSecArr = [];
        dnsServerSecArr.forEach(function (itemSec) {
            fullDnsServerSecArr.push(itemSec);
        })

        if (fullDnsServerSecArr.length > 0) {
            $('#resp_data_text_sec').val('');

            for (let i = 0; i < fullDnsServerSecArr.length; i++) {
                let itemSec = fullDnsServerSecArr[i];
                let circleBarSecId = `circleBar_sec_${i}`;
                let bar = `
                <svg id="${circleBarSecId}" width="30" height="30" viewBox="0 0 120 120">
  <circle cx="60" cy="60" r="50" fill="none" stroke="#e6e6e6" stroke-width="10"/>
  <circle cx="60" cy="60" r="50" fill="none" stroke="#3498db" stroke-width="10"
          stroke-dasharray="78.5 235.6" stroke-linecap="round">
    <animateTransform attributeName="transform"
                      type="rotate"
                      from="0 60 60"
                      to="360 60 60"
                      dur="1s"
                      repeatCount="indefinite"/>
  </circle>
</svg>
                `;

                $("#resp_data_text_sec").hide();
                $("#sec_box").append(bar);


                if (!isEmpty(itemSec)) {
                    $.ajax({
                        url: '/v1/query_dnssec',
                        type: 'POST',
                        async: true,
                        contentType: 'application/json', // 请求体格式为 JSON
                        data: JSON.stringify({
                            domain: domainSec,
                            recordType: recordTypeSec,
                            dnsServer: itemSec
                        }),
                        success: function (responseSec) {
                            $(`#${circleBarSecId}`).remove();
                            $("#resp_data_text_sec").show();
                            if (responseSec.code !== "1") {
                                $('#resp_data_text_sec').val(responseSec.msg);
                            } else {
                                //成功
                                //-----
                                let dataSec = responseSec.data;
                                let dItemSec = getDnsDesc(dataSec.dnsServer);
                                let temp_html = isEmpty(dItemSec)
                                    ? `<b style="color: #337ab7">${dataSec.dnsServer}</b>`
                                    : dItemSec;
                                if (dataSec && dataSec.record && dataSec.record.length > 0) {
                                    dataSec.record.forEach(function (item) {
                                        var newContent = `${domainSec} ${item.ttl}  ${item.value}`;
                                        console.log(newContent);
                                        $('#resp_data_text_sec').val(function (i, val) {
                                            return val + newContent + "\n"; // 在现有内容后拼接
                                        });
                                    });
                                } else {
                                    $('#resp_data_text_sec').val('无记录');
                                }
                            }

                        },
                        error: function (xhr, status, error) {
                            $(`#${circleBarSecId}`).remove();
                            $("#resp_data_text_sec").show();
                            $('#resp_data_text_sec').val('请求失败: ' + status);
                            Swal.fire({
                                icon: "error",
                                text: "请求失败!: " + status,
                            });
                        }
                    });

                }

            }
        }

    }

    function v1_query(domain, cusDns, recordType, dnsServerArr) {
        let fullDnsServerArr = [];
        if (!isEmpty(cusDns)) {
            fullDnsServerArr.push(cusDns);
        }
        dnsServerArr.forEach(function (item) {
            fullDnsServerArr.push(item);
        })


        if (fullDnsServerArr.length > 0) {
            $('#resp_data_table tbody').empty();
            for (let i = 0; i < fullDnsServerArr.length; i++) {
                let item = fullDnsServerArr[i];
                let circleBarId = `circleBar_${i}`;
                let bar = `<tr id="${circleBarId}"> <td colspan="4"><svg width="30" height="30" viewBox="0 0 120 120">
  <circle cx="60" cy="60" r="50" fill="none" stroke="#e6e6e6" stroke-width="10"/>
  <circle cx="60" cy="60" r="50" fill="none" stroke="#3498db" stroke-width="10"
          stroke-dasharray="78.5 235.6" stroke-linecap="round">
    <animateTransform attributeName="transform"
                      type="rotate"
                      from="0 60 60"
                      to="360 60 60"
                      dur="1s"
                      repeatCount="indefinite"/>
  </circle>
</svg> </td>  </tr>`;
                $('#resp_data_table tbody').append(bar);
                if (!isEmpty(item)) {
                    $.ajax({
                        url: '/v1/query',
                        type: 'POST',
                        async: true,
                        contentType: 'application/json', // 请求体格式为 JSON
                        data: JSON.stringify({
                            domain: domain,
                            recordType: recordType,
                            dnsServer: item
                        }),
                        success: function (response) {
                            if (response.code !== "1") {
                                $(`#${circleBarId}`).html(`   
                                            <td><b style='color: red'>${item}</b></td>
                                            <td><b style='color: red'>${recordType.toUpperCase()}</b></td>
                                            <td><b style='color: red'>${response.msg}</b></td>
                                            <td><b style='color: red'>${response.msg}</b></td>
                                `);
                            } else {
                                $(`#${circleBarId}`).remove();
                                //成功
                                //-----
                                let data = response.data;
                                let dItem = getDnsDesc(data.dnsServer);
                                let temp_html = isEmpty(dItem)
                                    ? `<b style="color: #337ab7">${data.dnsServer}</b>`
                                    : dItem;
                                if (data && data.record && data.record.length > 0) {
                                    let ips = [];
                                    let ttls = [];
                                    data.record.forEach(function (record) {
                                        ips.push(record.value);
                                        ttls.push(record.ttl);
                                    });

                                    // 动态生成新的行
                                    let newRow = `<tr>
                                                            <td>${temp_html}</td>
                                                            <td>${data.recordType}</td>
                                                            <td>${ips.join('<br>')}</td>
                                                            <td>${ttls.join('<br>')}</td>
                                                         </tr>`;
                                    $('#resp_data_table tbody').append(newRow);
                                } else {

                                    let newRow = `<tr>
                                                            <td>${temp_html}</td>
                                                            <td>${data.recordType}</td>
                                                            <td colspan="2">无记录</td>

                                                         </tr>`;
                                    $('#resp_data_table tbody').append(newRow);
                                }
                                //----
                            }

                        },
                        error: function (xhr, status, error) {
                            $(`#${circleBarId}`).remove();
                            Swal.fire({
                                icon: "error",
                                text: "请求失败!: " + status,
                            });
                        }
                    });

                }

            }
        }
    }
})
;

